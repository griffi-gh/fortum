use rocket::form::Form;
use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};
use rocket_db_pools::Connection;
use rocket_dyn_templates::{Template, context};
use crate::db::MainDatabase;
use crate::common::template_vars::TemplateVars;
use crate::common::authentication::Authentication;

// TODO use guard to check access instead

#[get("/chat?<conversation>")]
pub async fn conversation(mut db: Connection<MainDatabase>, auth: Authentication, vars: TemplateVars, conversation: i32) -> Option<Template> {
  if !MainDatabase::check_access(&mut db, auth.user_id, conversation).await {
    return None;
  }
  let conversations = MainDatabase::get_conversation_list(&mut db, auth.user_id).await;
  let messages = MainDatabase::get_messages(&mut db, conversation).await;
  Some(Template::render("chat", context!{ conversation_id: conversation, conversations, messages, vars }))
}

#[get("/chat")]
pub async fn chat(mut db: Connection<MainDatabase>, auth: Authentication, vars: TemplateVars, message: Option<FlashMessage<'_>>) -> Template {
  let conversations = MainDatabase::get_conversation_list(&mut db, auth.user_id).await;
  Template::render("chat", context!{ vars, message, conversations })
}

#[derive(FromForm)]
pub struct NewConversationData { user_id: i32 }

#[post("/new_conversation", data = "<data>")]
pub async fn new_conversation(mut db: Connection<MainDatabase>, auth: Authentication, data: Form<NewConversationData>) -> Result<Redirect, Flash<Redirect>> {
  if data.user_id == auth.user_id {
    //TODO redirect somewhere else
    return Err(Flash::error(Redirect::to("/user"), "Can't start conversation with yourself"));
  }
  let id = MainDatabase::create_or_get_existing_conversation(&mut db, auth.user_id, data.user_id).await;
  Ok(Redirect::to(uri!(conversation(id))))
}
