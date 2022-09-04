use rocket::form::Form;
use rocket::response::{Redirect, Flash};
use rocket::http::CookieJar;
use rocket_db_pools::Connection;
use crate::db::MainDatabase;
use crate::common::get_handler_macros::define_get_handler;
use crate::common::utils::token_cookie;
use super::misc::rocket_uri_macro_success;

define_get_handler!(register, "/register", "register");

#[derive(FromForm)]
pub struct RegisterData<'a> {
  email: &'a str,
  username: &'a str,
  password: &'a str,
  repeat_password: Option<&'a str>,
}

#[post("/register", data = "<data>")]
pub async fn post_register(data: Form<RegisterData<'_>>, mut db: Connection<MainDatabase>, cookies: &CookieJar<'_>) -> Flash<Redirect> {
  if let Some(repeat) = data.repeat_password.as_ref() {
    if &data.password != repeat {
      return Flash::error(Redirect::to(uri!(register)), "Passwords don't match");
    }
  }
  match MainDatabase::register(&mut db, &data.email, &data.username, &data.password).await {
    Ok(token) => {
      cookies.add_private(token_cookie(token));
      Flash::success(Redirect::to(uri!(success)), "Sign up ;;; Your account was created successfully")
    }
    Err(error) => Flash::error(Redirect::to(uri!(register)), error)
  }
}
