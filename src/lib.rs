//! An [async-session](https://github.com/http-rs/async-session) backend implemented
//! using [sea-orm](https://github.com/SeaQL/sea-orm), heavily inspired by
//! [async-sqlx-session](https://github.com/jbr/async-sqlx-session).
//!
//! # Basic usage
//!
//! In the following example we create a [`DatabaseSessionStore`], which implements
//! the [`SessionStore`] trait from [`async_session`].
//!
//! ```rust,no_run
//! use async_sea_orm_session::migration::Migrator;
//! use async_sea_orm_session::DatabaseSessionStore;
//! use sea_orm::{Database, DatabaseConnection};
//! use sea_orm_migration::MigratorTrait;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), sea_orm::DbErr> {
//!     // Create a sea_orm::DatabaseConnection in the usual way.
//!     let db: DatabaseConnection =
//!         Database::connect("protocol://username:password@host/database").await?;
//!
//!     // Run the async_sea_orm_session migration to create the session table.
//!     Migrator::up(&db, None).await?;
//!
//!     // Finally create a DatabaseSessionStore that implements SessionStore.
//!     let store = DatabaseSessionStore::new(db);
//!     Ok(())
//! }
//! ```
//!
//! # Examples
//!
//! For examples see the README in the [repository](https://github.com/dcchut/async-sea-orm-session).
//!
//! # License
//!
//! Licensed under either of
//! * Apache License, Version 2.0
//! ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
//! * MIT license
//! ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)
//!
//! at your option.
#![cfg_attr(docsrs, feature(doc_cfg))]
use async_session::{async_trait, serde_json, SessionStore};
use sea_orm::prelude::*;
use sea_orm::{sea_query, ConnectionTrait, DatabaseConnection, StatementBuilder};
use sea_query::OnConflict;

#[cfg(feature = "migration")]
#[cfg_attr(docsrs, doc(cfg(feature = "migration")))]
pub mod migration;
pub mod prelude;
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
                    serde_json::to_value(&session)?.into(),
                ])?
                .on_conflict(
                    OnConflict::column(sessions::Column::Id)
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
