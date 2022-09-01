use rocket::request::FlashMessage;
use rocket_dyn_templates::{Template, context};
use rocket_db_pools::Connection;
use crate::db::MainDatabase;
use crate::common::template_vars::TemplateVars;

#[get("/post/<id>")]
pub async fn post(id: i32, vars: TemplateVars, mut db: Connection<MainDatabase>, message: Option<FlashMessage<'_>>) -> Template {
  let post = MainDatabase::get_post(&mut db, id).await;
  Template::render("post", context! { vars, post, message })
}
