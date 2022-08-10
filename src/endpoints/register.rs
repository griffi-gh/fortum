use rocket::form::Form;
use rocket::response::Redirect;
use rocket::http::{Cookie, CookieJar};
use rocket_db_pools::Connection;
use rocket_dyn_templates::{Template, context};
use crate::db::MainDatabase;
use crate::common::TemplateVars;

#[get("/register/success")]
pub fn register_success(vars: TemplateVars) -> Template {
  Template::render("register", context! { success: true, vars })
}

#[get("/register?<error>")]
pub fn register(error: Option<&str>, vars: TemplateVars) -> Template {
  Template::render("register", context! { error, vars })
}

#[derive(FromForm)]
pub struct RegisterData {
  email: String,
  username: String,
  password: String,
  repeat_password: Option<String>,
}

#[post("/register", data = "<data>")]
pub async fn post_register(data: Form<RegisterData>, mut db: Connection<MainDatabase>, cookies: &CookieJar<'_>) -> Redirect {
  if let Some(repeat) = data.repeat_password.as_ref() {
    if &data.password != repeat {
      return Redirect::to(uri!(register(error = Some("Passwords don't match"))));
    }
  }
  match MainDatabase::register(&mut db, &data.email, &data.username, &data.password).await {
    Ok(token) => {
      cookies.add_private(Cookie::build("auth", token).secure(true).finish());
      Redirect::to("/register/success")
    }
    Err(error) => Redirect::to(uri!(register(error = Some(error))))
  }
}
