use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Conversation<'a> {
  pub conversation_id: i32,
  pub user_a: Option<i32>,
  pub user_b: Option<i32>,
  pub user_a_username: Option<&'a str>,
  pub user_b_username: Option<&'a str>
}

#[derive(Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "message_direction_type")]
pub enum MessageDirection {
  #[sqlx(rename="a_to_b")]
  #[serde(rename="a_to_b")]
  AToB,
  #[sqlx(rename="b_to_a")]
  #[serde(rename="b_to_a")]
  BtoA,
}


#[derive(Serialize, Deserialize)]
pub struct Message<'a> {
  pub message_id: i32,
  pub messsage_direction: MessageDirection,
  pub content: &'a str,
  pub created_on: DateTime<Utc>,
  pub reply_to: Option<i32>,
  pub reply_to_content: Option<&'a str>,
  pub edited: bool
}
