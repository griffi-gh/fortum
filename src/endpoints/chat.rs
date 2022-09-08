use rocket_db_pools::Connection;
use rocket_dyn_templates::{Template, context};
use crate::db::MainDatabase;
use crate::common::template_vars::TemplateVars;
use crate::common::authentication::Authentication;

#[get("/chat")]
pub async fn chat(mut db: Connection<MainDatabase>, auth: Authentication, vars: TemplateVars) -> Template {
  let conversations = MainDatabase::get_conversation_list(&mut db, auth.user_id).await;
  Template::render("chat", context!{ vars, conversations })
}
