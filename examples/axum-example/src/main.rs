use anyhow::Result;
use async_sea_orm_session::prelude::*;
use axum::extract::{FromRequest, RequestParts};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{async_trait, Extension, Json, Router};
use sea_orm::{Database, DatabaseConnection};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tower_cookies::{Cookie, CookieManagerLayer, Cookies};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

const SESSION_COOKIE_NAME: &str = "AXUM_SESSION_COOKIE";
const HIT_COUNT_KEY: &str = "hit_count";

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "example_sessions=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // We use an in-memory sqlite database for the purposes of this example.
    let db: DatabaseConnection = Database::connect("sqlite::memory:").await?;

    // Create the store and create the tables required for storing session information.
    let store = DatabaseSessionStore::new(db.clone());
    Migrator::up(&db, None).await?;

    let app = Router::new()
        .route("/", get(handler))
        .layer(Extension(store))
        .layer(CookieManagerLayer::new());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

/// Extractor that provides an [`async_session::Session`] based on a cookie.
struct CookieSession(Session);

#[derive(Copy, Clone, Deserialize, Serialize)]
struct Count(i32);

async fn handler(
    cookies: Cookies,
    CookieSession(mut session): CookieSession,
    Extension(store): Extension<DatabaseSessionStore>,
) -> impl IntoResponse {
    // Get the current hit count out of the session, increment it, then put it back in the session.
    let mut count = session.get(HIT_COUNT_KEY).unwrap_or(Count(0));

    count.0 += 1;

    session
        .insert(HIT_COUNT_KEY, count)
        .expect("failed to insert hit count");

    // Store the session, then insert the returned value into a cookie.
    if let Some(cookie_value) = store
        .store_session(session)
        .await
        .expect("failed to store session")
    {
        cookies.add(Cookie::new(SESSION_COOKIE_NAME, cookie_value));
    }

    Json(count)
}

#[async_trait]
impl<B> FromRequest<B> for CookieSession
where
    B: Send,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request(req: &mut RequestParts<B>) -> std::result::Result<Self, Self::Rejection> {
        // Here we use `DatabaseSessionStore`, but really any implementor of
        // [`async_session::SessionStore`] would work here!
        let Extension(store) = Extension::<DatabaseSessionStore>::from_request(req)
            .await
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "session store not provided",
                )
            })?;

        // Get the session cookie and load the corresponding session.
        let cookies = Cookies::from_request(req).await?;
        if let Some(session_cookie) = cookies.get(SESSION_COOKIE_NAME) {
            if let Ok(Some(session)) = store.load_session(session_cookie.value().to_string()).await
            {
                return Ok(CookieSession(session));
            }
        }

        // Or if something went wrong create a new one instead
        Ok(CookieSession(Session::new()))
    }
}
