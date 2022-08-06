use rocket_dyn_templates::{Template, context};
use rocket_db_pools::Connection;
use crate::db::MainDatabase;
use crate::common::TemplateVars;

#[get("/user")]
pub async fn user_self(vars: TemplateVars) -> Template {
  Template::render("user", context! { vars, self_page: true })
}

#[get("/user/<id>")]
pub async fn user(vars: TemplateVars, id: u32, db: Connection<MainDatabase>) -> Template {
  let user = MainDatabase::get_user(db, id).await;
  Template::render("user", context! { vars, user })
}
