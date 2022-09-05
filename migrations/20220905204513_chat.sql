CREATE TABLE conversations (
  conversation_id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  user_a INTEGER,
  user_b INTEGER,
  FOREIGN KEY(user_a) 
    REFERENCES users(user_id)
    ON DELETE SET NULL,
  FOREIGN KEY(user_b) 
    REFERENCES users(user_id)
    ON DELETE SET NULL
);
CREATE TABLE messages (
  message_id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  message_direction BOOLEAN,
  conversation_id INTEGER,
  content VARCHAR(512),
  FOREIGN KEY(conversation_id) 
    REFERENCES conversations(conversation_id)
    ON DELETE CASCADE
);
