# async-sea-orm-session

[![Latest version](https://img.shields.io/crates/v/async-sea-orm-session)](https://crates.io/crates/async-sea-orm-session)
[![crates.io downloads](https://img.shields.io/crates/d/async-sea-orm-session)](https://crates.io/crates/async-sea-orm-session)
[![Build Status](https://img.shields.io/github/workflow/status/dcchut/async-sea-orm-session/Push%20action/main)](https://github.com/dcchut/async-sea-orm-session/actions)
![Apache/MIT2.0 License](https://img.shields.io/crates/l/async-sea-orm-session)

An [async-session](https://github.com/http-rs/async-session) backend implemented
using [sea-orm](https://github.com/SeaQL/sea-orm), heavily inspired by
[async-sqlx-session](https://github.com/jbr/async-sqlx-session).

## Basic usage

In the following example we create a `DatabaseSessionStore`, which implements
the `SessionStore` trait from [async-session](https://github.com/http-rs/async-session).

```rust
use async_sea_orm_session::migration::Migrator;
use async_sea_orm_session::DatabaseSessionStore;
use sea_orm::{Database, DatabaseConnection};
use sea_orm_migration::MigratorTrait;

#[tokio::main]
async fn main() -> Result<(), sea_orm::DbErr> {
    // Create a sea_orm::DatabaseConnection in the usual way.
    let db: DatabaseConnection =
        Database::connect("protocol://username:password@host/database").await?;
   
    // Run the async_sea_orm_session migration to create the session table.
    Migrator::up(&db, None).await?;
    
    // Finally create a DatabaseSessionStore that implements SessionStore.
    let store = DatabaseSessionStore::new(db);
    Ok(())
}
```

## Examples

TODO

## License

Licensed under either of
* Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license
  ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
