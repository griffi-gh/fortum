use rocket::form::Form;
use rocket::response::Redirect;
use rocket::http::{Cookie, CookieJar};
use rocket_db_pools::Connection;
use crate::db::MainDatabase;
use rocket_dyn_templates::{Template, context};

#[get("/register/success")]
pub fn register_success() -> Template {
  Template::render("register", context! {
    success: true
  })
}

#[get("/register?<error>")]
pub fn register(error: Option<&str>) -> Template {
  Template::render("register", context! {
    error
  })
}

#[derive(FromForm)]
pub struct RegisterData {
  email: String,
  username: String,
  password: String,
}

#[post("/register", data = "<data>")]
pub async fn post_register(data: Form<RegisterData>, db: Connection<MainDatabase>, cookies: &CookieJar<'_>) -> Redirect {
  match MainDatabase::register(db, &data.email, &data.username, &data.password).await {
    Ok(token) => {
      cookies.add_private(Cookie::build("auth", token).secure(true).finish());
      Redirect::to("/register/success")
    }
    Err(error) => Redirect::to(uri!(register(error = Some(error))))
  }
}
