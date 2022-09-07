use rocket::http::{Cookie, CookieJar};
use rocket::response::{Redirect, Flash};
use rocket::form::Form;
use rocket_db_pools::Connection;
use rocket_dyn_templates::{context, Template};
use crate::db::MainDatabase;
use crate::common::authentication::Authentication;
use crate::common::template_vars::TemplateVars;
use crate::consts::AUTH_COOKIE_NAME;

//TODO "soft deletion"

#[derive(FromForm)]
pub struct DeletionData<'a> {
  email: &'a str,
  password: &'a str,
  confirmation: String,
  delete_posts: bool
}

#[post("/delete_account", data = "<data>")]
pub async fn delete_account<'a>(cookies: &CookieJar<'_>, mut db: Connection<MainDatabase>, auth: Authentication, data: Form<DeletionData<'a>>) -> Result<Redirect, Flash<Redirect>> {
  if MainDatabase::login(&mut db, data.email, data.password).await.is_err() {
    return Err(Flash::error(Redirect::to(uri!("/user")), "Invalid email or password")); 
  }
  if data.confirmation.replace(['\"', ','], "").trim().to_lowercase() != "yes delete my account" {
    return Err(Flash::error(Redirect::to(uri!("/user")), "Invalid confirmation string")); 
  }
  if data.delete_posts {
    sqlx::query("DELETE FROM posts WHERE author = $1")
      .bind(auth.user_id)
      .execute(&mut *db).await.unwrap();
    sqlx::query("DELETE FROM comments WHERE author = $1")
      .bind(auth.user_id)
      .execute(&mut *db).await.unwrap();
  }
  sqlx::query("DELETE FROM users WHERE user_id = $1")
    .bind(auth.user_id)
    .execute(&mut *db).await.unwrap();
  cookies.remove_private(Cookie::named(AUTH_COOKIE_NAME));
  Ok(Redirect::to(uri!(sad)))
}

#[get("/sad")]
pub async fn sad(vars: TemplateVars) -> Template {
  Template::render("sad", context!{ vars })
}
