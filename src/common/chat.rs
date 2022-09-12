use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
// use rocket::tokio::join;
// use rocket::Request;
// use rocket::request::{FromRequest, Outcome};
// use rocket_db_pools::Connection;
// use crate::db::MainDatabase;
// use crate::common::authentication::Authentication;

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub struct UserPair(pub i32, pub i32);
impl UserPair {
  #[inline] pub fn reverse(self) -> Self { Self(self.1, self.0) }
  #[inline] pub fn eq(self) -> bool { self.0 == self.1 }
}
impl From<(i32, i32)> for UserPair {
  #[inline] fn from(pair: (i32, i32)) -> Self { Self(pair.0, pair.1) }
}
impl From<UserPair> for (i32, i32) {
  #[inline] fn from(pair: UserPair) -> (i32, i32) { (pair.0, pair.1) }
}

// Conversation
#[derive(Serialize, Deserialize)]
pub struct Conversation {
  //id
  pub conversation_id: i32,
  //receiver user
  pub user_id: Option<i32>,
  pub user_profile_image: Option<String>,
  pub user_username: Option<String>,
  //last msg
  pub last_message: Option<String>,
}

// Message 

#[derive(Clone, Copy, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "message_direction_type")]
pub enum AbsoluteMessageDirection {
  #[sqlx(rename="a_to_b")]
  #[serde(rename="a_to_b")]
  AToB,
  #[sqlx(rename="b_to_a")]
  #[serde(rename="b_to_a")]
  BtoA
}

#[derive(Clone, Copy, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "message_direction_type")]
pub enum RelativeMessageDirection {
  #[sqlx(rename="a_to_b")]
  #[serde(rename="outgoing")]
  Outgoing,
  #[sqlx(rename="b_to_a")]
  #[serde(rename="incoming")]
  Incoming
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Message {
  pub message_id: i32,
  pub message_direction: AbsoluteMessageDirection,
  pub relative_message_direction: Option<RelativeMessageDirection>,
  pub content: String,
  pub created_on: DateTime<Utc>,
  pub reply_to: Option<i32>,
  pub reply_to_content: Option<String>,
  pub edited: bool
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MessageEventData {
  pub recv_user_id: i32,
  pub conversation_id: i32,
  pub message: Message,
}

// Authorization 

// pub struct ChatAuthorization;

// #[rocket::async_trait]
// impl<'r> FromRequest<'r> for ChatAuthorization {
//   type Error = ();
//   async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
//     let (auth, mut db) = join!(
//       req.guard::<Authentication>(),
//       req.guard::<Connection<MainDatabase>>()
//     );
//     let auth = auth.succeeded().unwrap();
//     let mut db = db.succeeded().unwrap();
//     sqlx::query_unchecked!(
//       r#"
//         SELECT 1 
//         FROM conversations 
//         WHERE (
//           ((user_a = $1) OR (user_b = $1)) AND 
//           (conversation_id = $2)
//         ) LIMIT 1;
//       "#,
//       auth.user_id,

//     )
//   }
// }
