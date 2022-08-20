#[macro_use] extern crate rocket;
#[macro_use] extern crate lazy_static;
use rocket::figment::{providers::Env, util::map};
use rocket::fs::FileServer;
use rocket_dyn_templates::Template;
use rocket_db_pools::Database;
use dotenv::dotenv;
use std::env;

pub mod db;
pub mod consts;
pub mod endpoints;
pub mod common;
mod cache_file_server;

use endpoints::{
  index::index,
  register::{register, post_register},
  login::{login, post_login},
  logout::{logout, super_logout},
  user::{user, user_self, user_self_fail},
  submit::{submit, submit_post, submit_post_error},
  post::post,
  vote::vote,
  dyn_profile_image::profile_image,
  error::{default_catcher, display_error},
  delete_account::delete_account,
  update_username::update_username,
  topics::{topic, topics},
  topics_create::{topics_create, topics_create_get},
  misc::{sad, success}
};

#[launch]
fn rocket() -> _ {
  dotenv().ok();
  let db_url = env::var("DATABASE_URL").unwrap();
  let figment = rocket::Config::figment()
    .merge(Env::raw().only(&["PORT", "SECRET_KEY"]))
    .merge(("databases", map!["main" => map!["url" => db_url]]));
  rocket::custom(figment)
    .attach(db::MainDatabase::init())
    .attach(Template::fairing())
    .mount("/", routes![
      index, 
      register, post_register,
      login, post_login,
      logout, super_logout,
      user, user_self, user_self_fail,
      submit, submit_post, submit_post_error,
      post,
      vote,
      profile_image,
      display_error,
      delete_account,
      update_username,
      topic, topics,
      topics_create, topics_create_get,
      sad, success,
    ])
    .register("/", catchers![default_catcher])
    .mount("/static", FileServer::from("./static/"))
    .mount("/static/cached_long/", routes![cache_file_server::files_long])
    .mount("/static/cached_short/", routes![cache_file_server::files_short])
}
