CREATE TYPE role_type AS ENUM ('banned', 'unverified', 'user', 'moderator', 'admin');
CREATE TABLE users (
  user_id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  username VARCHAR(15) NOT NULL,
  email VARCHAR(255) UNIQUE NOT NULL,
  password_hash VARCHAR NOT NULL,
  created_on TIMESTAMP NOT NULL DEFAULT (now() AT TIME ZONE 'utc'),
  last_activity TIMESTAMP NOT NULL DEFAULT (now() AT TIME ZONE 'utc'),
  user_role role_type NOT NULL DEFAULT 'user',
  token VARCHAR(24) UNIQUE NOT NULL CHECK (length(token) = 24) --16 byte token = 24 byte base64
);
CREATE TABLE posts (
  post_id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  author INTEGER,
  title VARCHAR(255) NOT NULL,
  content VARCHAR(3000),
  created_on TIMESTAMP NOT NULL DEFAULT (now() AT TIME ZONE 'utc'),
  score INTEGER,
  FOREIGN KEY(author) 
    REFERENCES users(user_id)
    ON DELETE SET NULL
);
