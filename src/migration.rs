//! This module contains migrations to help adding the sessions table required for this crate.
//!
//! # Example - Single migration
//! If you don't need to run any other migrations the [`Migrator`] struct can be used directly
//!
//! ```rust
//! use async_sea_orm_session::migration::Migrator;
//! use async_sea_orm_session::DatabaseSessionStore;
//! use sea_orm::{Database, DatabaseConnection};
//! use sea_orm_migration::MigratorTrait;
//!
//! # async fn doctest() -> Result<(), Box<dyn std::error::Error>> {
//!
//! let db: DatabaseConnection =
//!     Database::connect("protocol://username:password@host/database").await?;
//!
//! // Run the async-sea-orm-session migration, if it hasn't already been run.
//! Migrator::up(&db, None).await?;
//! let store = DatabaseSessionStore::new(db);
//! # Ok(())
//! # }
//! ```
//!
//! # Example - Multiple migrations
//! The previous example isn't super useful because it doesn't allow you to run other migrations.
//! To resolve this you should include [`SessionTableMigration`] directly in your own
//! [`MigratorTrait`] implementation, as in the example below:
//!
//! ```rust,ignore
//! // In migrations/src/lib.rs
//! pub use sea_orm_migration::prelude::*;
//! use async_sea_orm_session::migration::SessionTableMigration;
//!
//! mod m20220101_000001_sample_migration;
//! mod m20220102_000002_some_other_migration;
//!
//! pub struct Migrator;
//!
//! #[async_trait::async_trait]
//! impl MigratorTrait for Migrator {
//!     fn migrations() -> Vec<Box<dyn MigrationTrait>> {
//!         vec![
//!             Box::new(m20220101_000001_sample_migration::Migration),
//!             Box::new(m20220102_000002_some_other_migration::Migration),
//!             Box::new(SessionTableMigration),
//!         ]
//!     }
//! }
//! ```
use async_session::async_trait;
use sea_orm_migration::prelude::*;

pub struct Migrator;

#[async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(SessionTableMigration)]
    }
}

pub struct SessionTableMigration;
impl MigrationName for SessionTableMigration {
    fn name(&self) -> &str {
        "create_session_table"
    }
}

#[async_trait]
impl MigrationTrait for SessionTableMigration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                sea_query::Table::create()
                    .table(Session::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Session::Id).text().not_null().primary_key())
                    .col(ColumnDef::new(Session::Session).json_binary().not_null())
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(sea_query::Table::drop().table(Session::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(Iden)]
pub enum Session {
    Table,
    Id,
    Session,
}
