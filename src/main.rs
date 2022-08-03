#[macro_use] extern crate rocket;
#[macro_use] extern crate lazy_static;
use rocket::figment::{providers::Env, util::map};
use rocket::fs::FileServer;
use rocket_db_pools::Database;
use rocket_dyn_templates::Template;
use dotenv::dotenv;
use std::env;

pub mod consts;
pub mod endpoints;

use endpoints::{
  api::{
    register::register as api_register,
    login::login as api_login,
    logout::logout as api_logout,
  },
  frontend::{
    index::index as fe_index,
    login::login as fe_login,
    register::register as fe_register,
  }
};

#[derive(Database)]
#[database("main")]
pub struct MainDatabase(sqlx::PgPool);

#[launch]
fn rocket() -> _ {
  dotenv().ok();
  let db_url = env::var("DATABASE_URL").unwrap();
  let figment = rocket::Config::figment()
    .merge(Env::raw().only(&["PORT", "SECRET_KEY"]))
    .merge(("databases", map!["main" => map!["url" => db_url]]));
  rocket::custom(figment)
    .attach(MainDatabase::init())
    .attach(Template::fairing())
    .mount("/", routes![fe_index, fe_register, fe_login])
    .mount("/api", routes![api_register, api_login, api_logout])
    .mount("/static", FileServer::from("./static/"))
}
