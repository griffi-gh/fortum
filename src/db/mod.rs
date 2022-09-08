use rocket_db_pools::Database;
use rocket::{fairing, Rocket, Build};
use sqlx::PgPool;

/// Do not use outside of ./db/*.rs modules
pub mod inner_prelude {
  pub use rocket_db_pools::Connection;
  pub use super::MainDatabase;
  pub use crate::common::utils::executor;
  pub use sqlx::{self, Row};
}
mod chat;
mod account;
mod user;
mod posts;
mod stats;

pub async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
  let db = MainDatabase::fetch(&rocket).unwrap();
  let pool = &db.0;
  sqlx::migrate!().run(pool).await.unwrap();
  Ok(rocket)
}

#[derive(Database)]
#[database("main")]
pub struct MainDatabase(PgPool);
