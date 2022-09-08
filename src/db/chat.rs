use super::inner_prelude::*;
use crate::common::chat::ConversationListItem;

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
}
