use rocket_dyn_templates::{Template, context};
use rocket_db_pools::Connection;
use crate::db::MainDatabase;
use crate::common::TemplateVars;

#[get("/user/<id>")]
pub async fn user(vars: TemplateVars, id: Option<u32>, db: Connection<MainDatabase>) -> Template {
  let user = match id {
    Some(id) => MainDatabase::get_user(db, id).await,
    None => None
  };
  Template::render("user", context! { vars, user, self_page: id.is_none() })
}
