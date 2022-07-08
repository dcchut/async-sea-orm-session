//! This module exports [`DatabaseSessionStore`] and migration utilities.  In addition,
//! useful traits and types from both [`async_session`] and [`sea_orm_migration`] are re-rexported.
pub use crate::DatabaseSessionStore;
pub use async_session::{Session, SessionStore};

#[cfg(feature = "migration")]
#[cfg_attr(docsrs, doc(cfg(feature = "migration")))]
pub use sea_orm_migration::prelude::*;

#[cfg(feature = "migration")]
#[cfg_attr(docsrs, doc(cfg(feature = "migration")))]
pub use crate::migration::{Migrator, SessionTableMigration};
