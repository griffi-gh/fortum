use super::inner_prelude::*;
use crate::common::chat::{ConversationListItem, Message};

impl MainDatabase {
  pub async fn get_conversation_list(db: &mut Connection<Self>, user_id: i32) -> Vec<ConversationListItem> {
    sqlx::query_as!(ConversationListItem, r#"
      SELECT 
        conversations.conversation_id,
        users.user_id AS "user_id?",
        users.username AS "user_username?",
        users.profile_image AS "user_profile_image?",
        (
          SELECT content
          FROM messages AS msg
          WHERE msg.conversation_id = conversations.conversation_id
          ORDER BY msg.created_on DESC
          LIMIT 1
        ) as "last_message?"
      FROM conversations
      LEFT JOIN users 
        ON users.user_id = (
          CASE WHEN 
            conversations.user_a = $1 
          THEN 
            conversations.user_b
          ELSE 
            conversations.user_a 
          END
        )
      WHERE (user_a = $1 OR user_b = $1)
    "#, user_id).fetch_all(executor(db)).await.unwrap()
  }

  pub async fn create_or_get_existing_conversation(db: &mut Connection<Self>, user_a_id: i32, user_b_id: i32) -> i32 {
    let existing = sqlx::query(r#"
        SELECT conversation_id FROM conversations WHERE (user_a = $1 AND user_b = $2) OR (user_a = $2 AND user_b = $1)
      "#)
      .bind(user_a_id)
      .bind(user_b_id)
      .fetch_optional(executor(db)).await.unwrap();
    if let Some(existing) = existing {
      return existing.get(0);
    }
    sqlx::query(r#"
        INSERT INTO conversations (user_a, user_b) VALUES($1, $2) RETURNING conversation_id
      "#)
      .bind(user_a_id)
      .bind(user_b_id)
      .fetch_one(executor(db)).await.unwrap()
      .get(0)
  }

  pub async fn get_messages(db: &mut Connection<Self>, conversation_id: i32) -> Vec<Message> {
    sqlx::query_as!(Message, r#"
      SELECT 
        message_id,
        message_direction AS "message_direction: _",
        content,
        created_on,
        edited,
        reply_to,
        (
          SELECT content
          FROM messages AS msg
          WHERE msg.message_id = msg.reply_to
          LIMIT 1
        ) as "reply_to_content?"
        
      FROM messages
      WHERE conversation_id = $1
    "#, conversation_id).fetch_all(executor(db)).await.unwrap()
  }

  pub async fn check_access(db: &mut Connection<Self>, user_id: i32, conversation_id: i32) -> bool {
    sqlx::query(r#"
        SELECT 1 
        FROM conversations 
        WHERE (
          ((user_a = $1) OR (user_b = $1)) AND 
          (conversation_id = $2)
        ) LIMIT 1;
      "#)
      .bind(user_id).bind(conversation_id)
      .fetch_optional(executor(db)).await.unwrap().is_some()
  }
}
