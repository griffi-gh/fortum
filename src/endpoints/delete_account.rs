use rocket::http::{Cookie, CookieJar};
use rocket::response::Redirect;
use rocket_db_pools::Connection;
use crate::db::MainDatabase;
use crate::common::Authentication;

#[post("/post/delete_account")]
pub async fn delete_account() {}
