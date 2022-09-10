use rocket::form::Form;
use rocket::http::Status;
use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};
use rocket_db_pools::Connection;
use rocket_dyn_templates::{Template, context};
use crate::db::MainDatabase;
use crate::common::template_vars::TemplateVars;
use crate::common::authentication::Authentication;

// TODO use guard to check access instead

#[get("/chat?<conversation>")]
pub async fn conversation(
  mut db: Connection<MainDatabase>,
  auth: Authentication,
  vars: TemplateVars,
  conversation: i32,
  err: Option<FlashMessage<'_>>
) -> Result<Template, Flash<Redirect>> {
  // if !MainDatabase::check_access(&mut db, auth.user_id, conversation).await {
  //   return None;
  // }
  let conversation_obj = match MainDatabase::get_conversation(&mut db, auth.user_id, conversation).await {
    Ok(conversation) => conversation,
    Err(message) => return Err(Flash::error(Redirect::to(uri!(chat)), message))
  };
  let conversations = MainDatabase::get_conversation_list(&mut db, auth.user_id).await;
  let messages = MainDatabase::get_messages(&mut db, conversation, Some(auth.user_id)).await;
  Ok(Template::render("chat", context!{
    conversation_id: conversation,
    conversation: conversation_obj,
    conversations,  
    messages, 
    vars,
    message: err
  }))
}

#[get("/chat")]
pub async fn chat(mut db: Connection<MainDatabase>, auth: Authentication, vars: TemplateVars, message: Option<FlashMessage<'_>>) -> Template {
  let conversations = MainDatabase::get_conversation_list(&mut db, auth.user_id).await;
  Template::render("chat", context!{ vars, message, conversations })
}

#[derive(FromForm)]
pub struct NewConversationData {
  user_id: i32
}

#[post("/chat/new_conversation", data = "<data>")]
pub async fn new_conversation(mut db: Connection<MainDatabase>, auth: Authentication, data: Form<NewConversationData>) -> Result<Redirect, Flash<Redirect>> {
  let id = match MainDatabase::create_or_get_existing_conversation(&mut db, (auth.user_id, data.user_id).into()).await {
    Ok(id) => id, 
    Err(message) => return Err(Flash::error(Redirect::to(uri!(chat)), message))
  };
  Ok(Redirect::to(uri!(conversation(conversation = id))))
}

#[derive(FromForm)]
pub struct SendMessageData<'a> {
  conversation_id: i32,
  content: &'a str,
  reply_to: Option<i32>
}

#[post("/chat/send_message_form", data = "<data>")]
pub async fn send_message_form(mut db: Connection<MainDatabase>, auth: Authentication, data: Form<SendMessageData<'_>>) -> Result<Redirect, Flash<Redirect>> {
  let url = uri!(conversation(conversation = data.conversation_id));
  if let Err(message) = MainDatabase::send_message(&mut db, auth.user_id, data.content, data.conversation_id, data.reply_to).await {
    return Err(Flash::error(Redirect::to(url), message));
  }
  Ok(Redirect::to(url))
}

#[post("/chat/send_message", data = "<data>")]
pub async fn send_message(mut db: Connection<MainDatabase>, auth: Authentication, data: Form<SendMessageData<'_>>) -> Result<String, (Status, &'static str)> {
  match MainDatabase::send_message(&mut db, auth.user_id, data.content, data.conversation_id, data.reply_to).await {
    Ok(message_id) => Ok(message_id.to_string()),
    Err(message)  => Err((Status::BadRequest, message))
  }
}
