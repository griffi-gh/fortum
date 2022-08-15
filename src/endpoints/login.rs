use rocket_dyn_templates::{Template, context};
use rocket::form::Form;
use rocket::http::{Cookie, CookieJar};
use rocket::response::{Redirect, Flash};
use rocket::request::FlashMessage;
use rocket_db_pools::Connection;
use crate::db::MainDatabase;
use crate::common::TemplateVars;

#[get("/login")]
pub fn login(vars: TemplateVars, error: Option<FlashMessage>) -> Template {
  Template::render("login", context! { error, vars })
}

#[derive(FromForm)]
pub struct LoginData<'a> {
  email: &'a str,
  password:  &'a str,
}

#[post("/login", data = "<data>")]
pub async fn post_login(data: Form<LoginData<'_>>, mut db: Connection<MainDatabase>, cookies: &CookieJar<'_>) -> Result<Redirect, Flash<Redirect>> {
  match MainDatabase::login(&mut db, &data.email, &data.password).await {
    Ok(token) => {
      cookies.add_private(Cookie::build("auth", token).secure(true).finish());
      Ok(Redirect::to(uri!("/")))
    }
    Err(error) => {
      Err(Flash::error(Redirect::to(uri!(login)), error))
    }
  }
}
