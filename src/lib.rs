//! # async-sea-orm-session
//!
use async_session::{async_trait, serde_json, SessionStore};
use sea_orm::prelude::*;
use sea_orm::{sea_query, ConnectionTrait, DatabaseConnection, StatementBuilder};
use sea_query::OnConflict;

mod sessions;

use sessions::Entity as Session;

#[derive(Clone, Debug)]
pub struct DatabaseSessionStore {
    connection: DatabaseConnection,
}

impl DatabaseSessionStore {
    /// Create a new [`DatabaseSessionStore`] from the given [`DatabaseConnection`].
    ///
    /// # Example
    /// ```rust
    /// use async_sea_orm_session::DatabaseSessionStore;
    /// use sea_orm::{Database, DatabaseConnection};
    /// # async fn doctest() -> Result<(), Box<dyn std::error::Error>> {
    ///
    /// let db: DatabaseConnection =
    ///     Database::connect("protocol://username:password@host/database").await?;
    ///
    /// // Make a `DatabaseSessionStore` which reuses the underlying connection pool:
    /// let store = DatabaseSessionStore::new(db.clone());
    ///
    /// // Alternatively if you don't mind moving `db` you can do:
    /// let store = DatabaseSessionStore::new(db);
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(connection: DatabaseConnection) -> DatabaseSessionStore {
        Self { connection }
    }
}

#[async_trait]
impl SessionStore for DatabaseSessionStore {
    async fn load_session(
        &self,
        cookie_value: String,
    ) -> async_session::Result<Option<async_session::Session>> {
        let id = async_session::Session::id_from_cookie_value(&cookie_value)?;
        Ok(Session::find_by_id(id)
            .one(&self.connection)
            .await?
            .map(|m| serde_json::from_value(m.session))
            .transpose()?)
    }

    async fn store_session(
        &self,
        session: async_session::Session,
    ) -> async_session::Result<Option<String>> {
        let stmt = StatementBuilder::build(
            sea_query::Query::insert()
                .into_table(Session)
                .columns(vec![sessions::Column::Id, sessions::Column::Session])
                .values(vec![
                    session.id().into(),
                    serde_json::to_string(&session)?.into(),
                ])?
                .on_conflict(
                    OnConflict::new()
                        .update_columns([sessions::Column::Session])
                        .to_owned(),
                ),
            &self.connection.get_database_backend(),
        );

        self.connection.execute(stmt).await?;
        Ok(session.into_cookie_value())
    }

    async fn destroy_session(&self, session: async_session::Session) -> async_session::Result {
        Session::delete_by_id(session.id().to_string())
            .exec(&self.connection)
            .await?;
        Ok(())
    }

    async fn clear_store(&self) -> async_session::Result {
        Session::delete_many().exec(&self.connection).await?;
        Ok(())
    }
}
