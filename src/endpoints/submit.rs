use rocket::form::Form;
use rocket::request::FlashMessage;
use rocket::response::{Redirect, Flash};
use rocket_db_pools::Connection;
use rocket_dyn_templates::{Template, context};
use crate::db::MainDatabase;
use crate::common::{Authentication, TemplateVars};
use super::login::rocket_uri_macro_login;
use super::post::rocket_uri_macro_post;

#[derive(FromForm)]
pub struct PostSubmitData<'a> {
  title: String,
  body: Option<&'a str>,
  topic: i32,
}

#[get("/submit?<topic>")]
pub async fn submit<'a>(vars: TemplateVars, error: Option<FlashMessage<'a>>, topic: Option<i32>) -> Template {
  Template::render("submit", context! { vars, error, topic } )
}

#[post("/submit", data = "<data>")]
pub async fn submit_post(data: Form<PostSubmitData<'_>>, mut db: Connection<MainDatabase>, auth: Authentication) -> Result<Redirect, Flash<Redirect>> {
  match MainDatabase::submit_post(&mut db, Some(auth.user_id), data.topic, &data.title, data.body).await {
    Ok(id) => Ok(Redirect::to(uri!(post(id = id)))),
    Err(err) => Err(Flash::error(Redirect::to(uri!(submit(topic = Some(data.topic)))), err)),
  }
}
#[post("/submit", rank = 2)]
pub async fn submit_post_error() -> Flash<Redirect> {
  Flash::error(Redirect::to(uri!(login)), "Log in before posting")
}
