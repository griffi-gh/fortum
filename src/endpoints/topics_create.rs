use rocket::response::{Flash, Redirect};
use rocket::form::Form;
use crate::common::authentication::Authentication;
use crate::common::get_handler_macros::define_get_handler;
use super::topics::rocket_uri_macro_topic;

define_get_handler!(topics_create_get, "/topics/create", "topics_create");

#[derive(FromForm)]
pub struct CreateTopicData<'a> {
  pub name: &'a str,
  pub description: Option<&'a str>
}

#[post("/topics/create", data = "<data>")]
pub async fn topics_create(data: Form<CreateTopicData<'_>>, _auth: Authentication) -> Flash<Redirect> {
  Flash::success(Redirect::to(uri!(topic(name = data.name, page = Option::<u32>::None))), "Topic created successfully")
}
