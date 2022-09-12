use rocket::{Shutdown, State};
use rocket::form::Form;
use rocket::http::Status;
use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};
use rocket::response::stream::{Event, EventStream};
use rocket::serde::{Serialize, Deserialize};
use rocket::serde::json::{Json, Value, json};
use rocket::tokio::select;
use rocket::tokio::sync::broadcast::{Sender, error::RecvError};
use rocket_db_pools::Connection;
use rocket_dyn_templates::{Template, context};
use crate::common::chat::MessageEventData;
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
  let messages = match MainDatabase::get_messages(&mut db, conversation, Some(auth.user_id)).await{
    Ok(messages) => messages,
    Err(message) => return Err(Flash::error(Redirect::to(uri!(chat)), message))
  };
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

#[derive(FromForm, Serialize, Deserialize)]
pub struct SendMessageData<'a> {
  conversation_id: i32,
  content: &'a str,
  reply_to: Option<i32>
}

pub async fn send_message(mut db: Connection<MainDatabase>, auth: Authentication, mut data: SendMessageData<'_>, queue: &State<Sender<MessageEventData>>) -> Result<i32, &'static str> {
  //Trim message content
  data.content = data.content.trim();
  match MainDatabase::send_message(&mut db, auth.user_id, data.content, data.conversation_id, data.reply_to).await {
    Ok(message_id) => {
      //two checks/db requests :p
      if let Some(o_user_id) = MainDatabase::get_other_conv_user(&mut db, auth.user_id, data.conversation_id).await {
        if let Some(message) = MainDatabase::get_message(&mut db, message_id).await {
          //Only fails if no subscribers so we can safely ignore the error :p
          let _ = queue.send(MessageEventData {
            recv_user_id: o_user_id,
            conversation_id: data.conversation_id,
            message
          });
        } 
      }
      Ok(message_id)
    },
    Err(message) => Err(message)
  }
}

#[post("/chat/send_message_form", data = "<data>")]
pub async fn send_message_form(db: Connection<MainDatabase>, auth: Authentication, data: Form<SendMessageData<'_>>, queue: &State<Sender<MessageEventData>>) -> Result<Redirect, Flash<Redirect>> {
  let url = uri!(conversation(conversation = data.conversation_id));
  match send_message(db, auth, data.into_inner(), queue).await {
    Ok(_) => Ok(Redirect::to(url)),
    Err(message) => Err(Flash::error(Redirect::to(url), message))
  }
}

#[post("/chat/send_message", format = "json", data = "<data>")]
pub async fn send_message_json(db: Connection<MainDatabase>, auth: Authentication, data: Json<SendMessageData<'_>>, queue: &State<Sender<MessageEventData>>) -> Result<Value, (Status, Value)> {
  match send_message(db, auth, data.into_inner(), queue).await {
    Ok(message_id) => Ok(json!({
      "code": 200,
      "success": true,
      "message_id": message_id
    })),
    Err(message) => Err((Status::BadRequest, json!({
      "code": 400,
      "success": false,
      "error": message
    })))
  }
}

#[get("/chat/events")]
pub async fn events(auth: Authentication, queue: &State<Sender<MessageEventData>>, mut end: Shutdown) -> EventStream![] {
  let mut rx = queue.subscribe();
  EventStream! {
    loop {
      let msg = select! {
        msg = rx.recv() => match msg {
          Ok(msg) => msg,
          Err(RecvError::Closed) => break,
          Err(RecvError::Lagged(_)) => continue,
        },
        _ = &mut end => break,
      };
      if msg.recv_user_id == auth.user_id {
        yield Event::json(&msg).event("new_message");
      }
    }
  }
}
