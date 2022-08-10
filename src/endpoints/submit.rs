use rocket::form::Form;
use rocket::response::Redirect;
use rocket_db_pools::Connection;
use rocket_dyn_templates::{Template, context};
use crate::db::MainDatabase;
use crate::common::{Authentication, TemplateVars};
use crate::endpoints::login::rocket_uri_macro_login;
use crate::endpoints::post::rocket_uri_macro_post;

#[derive(FromForm)]
pub struct PostSubmitData<'a> {
  title: String,
  body: Option<&'a str>,
  topic: i32,
}

#[get("/submit?<error>")]
pub async fn submit(error: Option<&str>, vars: TemplateVars) -> Template {
  Template::render("submit", context! { vars, error } )
}

#[post("/submit", data = "<data>")]
pub async fn submit_post(data: Form<PostSubmitData<'_>>, mut db: Connection<MainDatabase>, auth: Authentication) -> Redirect {
  match MainDatabase::submit_post(&mut db, Some(auth.user_id.unwrap()), data.topic, &data.title, data.body).await {
    Ok(id) => Redirect::to(uri!(post(id = id, success = true))),
    Err(err) => Redirect::to(uri!(submit(error = Some(err)))),
  }
}

#[post("/submit", rank = 2)]
pub async fn submit_post_error() -> Redirect {
  return Redirect::to(uri!(login(error = Some("Log in before posting stuff"))));
}
