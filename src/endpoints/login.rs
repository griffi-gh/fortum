use rocket_dyn_templates::{Template, context};
use rocket::form::Form;
use rocket::http::{Cookie, CookieJar};
use rocket::response::Redirect;
use rocket_db_pools::Connection;
use crate::db::MainDatabase;

#[get("/login?<error>")]
pub fn login(error: Option<&str>) -> Template {
  Template::render("login", context! {
    error
  })
}

#[derive(FromForm)]
pub struct LoginData {
  email: String,
  password: String,
}

#[post("/login", data = "<data>")]
pub async fn post_login(data: Form<LoginData>, db: Connection<MainDatabase>, cookies: &CookieJar<'_>) -> Redirect {
  match MainDatabase::login(db, &data.email, &data.password).await {
    Ok(token) => {
      cookies.add_private(Cookie::build("auth", token).secure(true).finish());
      Redirect::to(uri!("/"))
    }
    Err(error) => Redirect::to(uri!(login(error = Some(error))))
  }
}
