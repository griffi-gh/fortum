use rocket::form::Form;
use rocket::response::Redirect;
use rocket_db_pools::Connection;
use rocket_dyn_templates::{Template, context};
use crate::db::MainDatabase;
use crate::common::{Authentication, TemplateVars};

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
pub async fn submit_post(data: Form<PostSubmitData<'_>>, db: Connection<MainDatabase>, auth: Authentication) -> Redirect {
  if !auth.valid { return Redirect::to("/login?error=Log in to post stuff"); }
  match MainDatabase::submit_post(db, Some(auth.user_id.unwrap() as i32), data.topic, &data.title, data.body).await {
    Ok(_) => Redirect::to(uri!("/post/todo_insert_id_here?success")),
    Err(err) => Redirect::to(uri!(submit(error = Some(err)))),
  }
}
