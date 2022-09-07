use rocket_db_pools::Connection;
use rocket_dyn_templates::{Template, context};
use crate::db::MainDatabase;
use crate::common::template_vars::TemplateVars;

#[get("/chat")]
pub async fn chat(mut _db: Connection<MainDatabase>, vars: TemplateVars) -> Template {
  Template::render("chat", context!{ vars })
}
