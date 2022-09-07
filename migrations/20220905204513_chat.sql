CREATE TYPE message_direction_type AS ENUM ('a_to_b', 'b_to_a');
CREATE TABLE IF NOT EXISTS conversations (
  conversation_id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  user_a INTEGER REFERENCES users ON DELETE SET NULL,
  user_b INTEGER REFERENCES users ON DELETE SET NULL
);
CREATE TABLE IF NOT EXISTS messages (
  message_id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  message_direction message_direction_type NOT NULL,
  conversation_id INTEGER NOT NULL REFERENCES conversations ON DELETE CASCADE,
  content VARCHAR(512) NOT NULL,
  created_on TIMESTAMPTZ NOT NULL DEFAULT now(),
  reply_to INTEGER REFERENCES messages ON DELETE SET NULL,
  edited BOOLEAN NOT NULL DEFAULT false
);
