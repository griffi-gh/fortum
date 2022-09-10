use super::inner_prelude::*;
use crate::common::chat::{Conversation, Message, UserPair};
use crate::consts::MAX_CHAT_MESSAGE_LEN;

impl MainDatabase {
  /// DOESN'T SET `last_message`!!!
  pub async fn get_conversation(db: &mut Connection<Self>, user_id: i32, conversation_id: i32) -> Result<Conversation, &'static str> {
    if !Self::check_access(db, user_id, conversation_id).await {
      return Err("You dont have access to this conversation");
    }
    let conversation = sqlx::query_as!(Conversation, r#"
        SELECT
          conversations.conversation_id,
          users.user_id AS "user_id?",
          users.username AS "user_username?",
          users.profile_image AS "user_profile_image?",
          null AS "last_message?"
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
        WHERE (user_a = $1 OR user_b = $1) AND (conversation_id = $2)
      "#, user_id, conversation_id).fetch_optional(executor(db)).await.unwrap();
    match conversation {
      Some(v) => Ok(v),
      None => Err("Conversation doesn't exist")
    }
  }
  
  pub async fn get_conversation_list(db: &mut Connection<Self>, user_id: i32) -> Vec<Conversation> {
    sqlx::query_as!(Conversation, r#"
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

  pub async fn create_or_get_existing_conversation(db: &mut Connection<Self>, users: UserPair) -> Result<i32, &'static str> {
    if users.eq() {
      return Err("Can't start conversation with yourself");
    }
    // Existing chat
    {
      let existing = sqlx::query(r#"
          SELECT conversation_id FROM conversations WHERE (user_a = $1 AND user_b = $2) OR (user_a = $2 AND user_b = $1)
        "#)
        .bind(users.0)
        .bind(users.1)
        .fetch_optional(executor(db)).await.unwrap();
      if let Some(existing) = existing {
        return Ok(existing.get(0));
      }
    }
    // Create new one
    Ok(
      sqlx::query(r#"
        INSERT INTO conversations (user_a, user_b) VALUES($1, $2) RETURNING conversation_id
      "#)
      .bind(users.0)
      .bind(users.1)
      .fetch_one(executor(db)).await.unwrap()
      .get(0)
    )
  }

  //UserPair is required for relative direction
  pub async fn get_messages(db: &mut Connection<Self>, conversation_id: i32, user_id: Option<i32>) -> Result<Vec<Message>, &'static str> {
    if let Some(user_id) = user_id {
      if !Self::check_access(db, user_id, conversation_id).await {
        return Err("You dont have access to this conversation");
      }
    }
    Ok(
      sqlx::query_as!(Message, r#"
          SELECT 
            message_id,
            message_direction AS "message_direction!: _",
            (
              CASE 
                -- IF THE ID IS -1 OR NULL THEN SKIP AND RETURN null
                WHEN $2 = -1 THEN null
                WHEN $2 = null THEN null
                ELSE (
                  CASE  
                    WHEN ( -- IF user_a IN conversations MATCHES user_id
                      (
                        SELECT user_a
                        FROM conversations 
                        WHERE conversation_id = $1
                      ) = $2 
                    ) THEN ( -- THEN PUT message_direction AS-IS INTO relative_message_direction  
                      message_direction
                    ) ELSE ( -- ELSE, INVERT IT
                      CASE 
                        WHEN (message_direction = 'a_to_b') THEN 'b_to_a'::message_direction_type
                        ELSE 'a_to_b'::message_direction_type
                      END
                    )
                  END
                )
              END
            ) AS "relative_message_direction?: _",
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
        "#, 
        conversation_id, 
        user_id.unwrap_or(-1)
      ).fetch_all(executor(db)).await.unwrap()
    )
  }

  pub async fn get_message(db: &mut Connection<Self>, message_id: i32) -> Option<Message> {
    todo!()
  }

  pub async fn send_message(db: &mut Connection<Self>, user_id: i32, content: &str, conversation_id: i32, reply_to: Option<i32>) -> Result<i32, &'static str> {
    // Trim content
    let content = content.trim();
    // Check message length
    if content.len() > MAX_CHAT_MESSAGE_LEN {
      return Err("Message is too long");
    } else if content.replace(['\n', '\r'], "").trim().is_empty() {
      return Err("Empty messages are not allowed");
    } 
    // Check conversation access
    if !Self::check_access(db, user_id, conversation_id).await {
      return Err("You dont have access to this conversation");
    }
    // Check reply id
    if let Some(reply_to) = reply_to {
      let reply_check = sqlx::query(r#"
        SELECT 1 FROM messages WHERE (
          (message_id = $1) AND
          (conversation_id = $2)
        ) LIMIT 1
      "#)
      .bind(reply_to).bind(conversation_id)
      .fetch_optional(executor(db)).await.unwrap().is_some();
      if !reply_check {
        return Err("Can't reply to a message that doesn't exist/from another chat")
      }
    }
    // Create message
    Ok(
      sqlx::query!(r#"
        INSERT INTO messages (
          message_direction,
          conversation_id,
          content,
          reply_to
        ) VALUES (
          CASE 
            WHEN (
              (
                SELECT user_a
                FROM conversations 
                WHERE conversation_id = $1
              ) = $2 
            ) THEN 'a_to_b'::message_direction_type
            ELSE 'b_to_a'::message_direction_type
          END,
          $1,
          $3,
          $4
        ) RETURNING message_id
      "#, user_id, conversation_id, content, reply_to)
        .fetch_one(executor(db)).await.unwrap().message_id
    )
  }
}
