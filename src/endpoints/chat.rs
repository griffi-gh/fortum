use rocket::form::Form;
use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};
use rocket_db_pools::Connection;
use rocket_dyn_templates::{Template, context};
use crate::db::MainDatabase;
use crate::common::template_vars::TemplateVars;
use crate::common::authentication::Authentication;

#[get("/chat")]
pub async fn chat(mut db: Connection<MainDatabase>, auth: Authentication, vars: TemplateVars, message: Option<FlashMessage<'_>>) -> Template {
  let conversations = MainDatabase::get_conversation_list(&mut db, auth.user_id).await;
  Template::render("chat", context!{ vars, message, conversations })
}

#[get("/chat/<id>")]
pub async fn conversation(mut db: Connection<MainDatabase>, auth: Authentication, id: i32) -> Template {
  todo!()
}

#[derive(FromForm)]
pub struct NewConversationData { user_id: i32 }

#[post("/new_conversation", data = "<data>")]
pub async fn new_conversation(mut db: Connection<MainDatabase>, auth: Authentication, data: Form<NewConversationData>) -> Result<Redirect, Flash<Redirect>> {
  let id = MainDatabase::create_or_get_existing_conversation(&mut db, auth.user_id, data.user_id).await;
  Ok(Redirect::to(uri!(conversation(id))))
}
