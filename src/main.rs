#![allow(clippy::unnecessary_lazy_evaluations)] // `FromForm` warnings fix :p 

#[macro_use] extern crate rocket;
use figment::{providers::Env, util::map};
use serde::{Serialize, Deserialize};
use rocket::tokio::sync::broadcast::channel;
use rocket::fairing::AdHoc;
use rocket_dyn_templates::Template;
use rocket_db_pools::Database;
use dotenvy::dotenv;
use std::env;

pub mod db;
pub mod consts;
pub mod endpoints;
pub mod common;
mod cache_file_server;

use common::chat::MessageEventData;
use db::{MainDatabase, run_migrations};
use cache_file_server::CacheFileServer;
use endpoints::{
  index::index,
  register::{register, post_register},
  login::{login, post_login},
  logout::{logout, super_logout},
  user::{user, user_self},
  submit::{submit, submit_post},
  post::post,
  vote::vote,
  dyn_profile_image::profile_image,
  delete_account::delete_account,
  update_username::update_username,
  topics::{topic, /* topics */},
  topics_create::{topics_create, topics_create_get},
  misc::{sad, success},
  error::{default_catcher, display_error},
  chat::{chat, conversation, new_conversation, send_message_json, send_message_form, events as chat_events}
};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
  versioning: u64,
  results_per_page: u32,
  cache_length: usize,
}

#[launch]
fn rocket() -> _ {
  dotenv().ok();
  //TODO extract DATABASE_URL from figment instead
  let db_url = env::var("DATABASE_URL").unwrap(); 
  let figment = rocket::Config::figment()
    .merge(Env::raw().only(&["ADDRESS", "PORT", "SECRET_KEY"]))
    .merge(("databases", map!["main" => map!["url" => db_url]]));
  let config: Config = figment.extract().unwrap();
  rocket::custom(figment)
    .attach(MainDatabase::init())
    .attach(AdHoc::try_on_ignite("DB Migrations", run_migrations))
    .attach(AdHoc::config::<Config>())
    .attach(Template::fairing())
    .manage(channel::<MessageEventData>(1024).0)
    .mount("/", routes![
      index,
      register, post_register,
      login, post_login,
      logout, super_logout,
      user, user_self,
      submit, submit_post,
      post,
      vote,
      profile_image,
      display_error,
      delete_account,
      update_username,
      topic, /* topics, */
      topics_create, topics_create_get,
      chat, conversation, new_conversation, send_message_json, send_message_form, chat_events,
      sad, success,
    ])
    .register("/", catchers![default_catcher])
    .mount("/static", CacheFileServer::new("./static", config.cache_length))
}
