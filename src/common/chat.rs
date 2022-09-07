use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "role_type")]
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
  pub conversation_id: i32,
  pub content: &'a str,
  pub created_on: DateTime<Utc>,
  pub reply_to: Option<i32>,
  pub reply_to_message: Option<&'a str>,
  pub edited: bool
}
