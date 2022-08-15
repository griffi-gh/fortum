use rocket::form::Form;
use rocket::request::FlashMessage;
use rocket::response::{Redirect, Flash};
use rocket::http::{Cookie, CookieJar};
use rocket_db_pools::Connection;
use rocket_dyn_templates::{Template, context};
use crate::db::MainDatabase;
use crate::common::TemplateVars;

#[get("/register/success")]
pub fn register_success(vars: TemplateVars) -> Template {
  Template::render("register", context! { success: true, vars })
}

#[get("/register")]
pub fn register(vars: TemplateVars, error: Option<FlashMessage>) -> Template {
  Template::render("register", context! { error, vars })
}

#[derive(FromForm)]
pub struct RegisterData<'a> {
  email: &'a str,
  username: &'a str,
  password: &'a str,
  repeat_password: Option<&'a str>,
}

#[post("/register", data = "<data>")]
pub async fn post_register(data: Form<RegisterData<'_>>, mut db: Connection<MainDatabase>, cookies: &CookieJar<'_>) -> Result<Redirect, Flash<Redirect>> {
  if let Some(repeat) = data.repeat_password.as_ref() {
    if &data.password != repeat {
      return Err(Flash::error(Redirect::to(uri!(register)), "Passwords don't match"));
    }
  }
  match MainDatabase::register(&mut db, &data.email, &data.username, &data.password).await {
    Ok(token) => {
      cookies.add_private(Cookie::build("auth", token).secure(true).finish());
      Ok(Redirect::to("/register/success"))
    }
    Err(error) => {
      return Err(Flash::error(Redirect::to(uri!(register)), error));
    }
  }
}
