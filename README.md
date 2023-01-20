# async-sea-orm-session

[![Latest version](https://img.shields.io/crates/v/async-sea-orm-session)](https://crates.io/crates/async-sea-orm-session)
[![crates.io downloads](https://img.shields.io/crates/d/async-sea-orm-session)](https://crates.io/crates/async-sea-orm-session)
[![Build Status](https://img.shields.io/github/actions/workflow/status/dcchut/async-sea-orm-session/rust.yml?branch=main)](https://github.com/dcchut/async-sea-orm-session/actions)
![Apache/MIT2.0 License](https://img.shields.io/crates/l/async-sea-orm-session)

An [async-session](https://github.com/http-rs/async-session) backend implemented
using [sea-orm](https://github.com/SeaQL/sea-orm), heavily inspired by
[async-sqlx-session](https://github.com/jbr/async-sqlx-session).

More information can be found in the [crate documentation](https://docs.rs/async-sea-orm-session).

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

There are full examples in the `examples` directory of the repository.  Feel free to contribute examples showing
different setups!

- **axum-example**

This example combines the [axum](https://github.com/tokio-rs/axum) web application
framework with `async-sea-orm-session` for session storage and [tower-cookies](https://github.com/imbolc/tower-cookies)
for cookie management.

By default, this example runs using an in-memory sqlite database.  The
example can also be run using a postgres database by running the following
from the `axum-example` subdirectory:

```shell
DATABASE_URI=postgres://username:password@host/database cargo run --features postgres
```


## License

Licensed under either of
* Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license
  ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
