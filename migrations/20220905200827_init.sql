DO $$
BEGIN
  IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'role_type') THEN
    CREATE TYPE role_type AS ENUM ('banned', 'unverified', 'user', 'moderator', 'admin');
  END IF;
END$$;
CREATE TABLE IF NOT EXISTS users (
  user_id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  username VARCHAR(25) NOT NULL,
  email VARCHAR(255) UNIQUE NOT NULL,
  password_hash VARCHAR NOT NULL,
  created_on TIMESTAMPTZ NOT NULL DEFAULT now(),
  last_activity TIMESTAMPTZ NOT NULL DEFAULT now(),
  user_role role_type NOT NULL DEFAULT 'unverified',
  token VARCHAR(24) UNIQUE NOT NULL CHECK (length(token) = 24), --16 byte token = 24 byte base64
  profile_image VARCHAR,
  banner_image VARCHAR,
  email_verify VARCHAR UNIQUE DEFAULT md5(random()::text)
);
CREATE TABLE IF NOT EXISTS  topics (
  topic_id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  topic_name VARCHAR(30) UNIQUE NOT NULL,
  topic_description VARCHAR(255),
  created_on TIMESTAMPTZ NOT NULL DEFAULT now(),
  creator INTEGER,
  FOREIGN KEY(creator) 
    REFERENCES users(user_id)
    ON DELETE SET NULL
);
CREATE TABLE IF NOT EXISTS  posts (
  post_id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  author INTEGER,
  title VARCHAR(255) NOT NULL,
  content VARCHAR(3000),
  created_on TIMESTAMPTZ NOT NULL DEFAULT now(),
  topic INTEGER NOT NULL,
  votes BIGINT NOT NULL DEFAULT 0,
  FOREIGN KEY(author) 
    REFERENCES users(user_id)
    ON DELETE SET NULL,
  FOREIGN KEY(topic) 
    REFERENCES topics(topic_id)
    ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS  votes (
  vote_id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  post_id INTEGER NOT NULL,
  user_id INTEGER,
  vote BOOLEAN NOT NULL,
  voted_on TIMESTAMPTZ NOT NULL DEFAULT now(),
  FOREIGN KEY(post_id) 
    REFERENCES posts(post_id)
    ON DELETE CASCADE,
  FOREIGN KEY(user_id) 
    REFERENCES users(user_id)
    ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS comments (
  comment_id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  author INTEGER,
  content VARCHAR(500) NOT NULL,
  created_on TIMESTAMPTZ NOT NULL DEFAULT now(),
  votes BIGINT NOT NULL DEFAULT 0,
  FOREIGN KEY(author) 
    REFERENCES users(user_id)
    ON DELETE SET NULL
);
CREATE TABLE IF NOT EXISTS comment_votes (
  vote_id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  comment_id INTEGER NOT NULL,
  user_id INTEGER,
  vote BOOLEAN NOT NULL,
  voted_on TIMESTAMPTZ NOT NULL DEFAULT now(),
  FOREIGN KEY(comment_id) 
    REFERENCES comments(comment_id)
    ON DELETE CASCADE,
  FOREIGN KEY(user_id) 
    REFERENCES users(user_id)
    ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS stats (
  users INTEGER NOT NULL DEFAULT 0,
  posts INTEGER NOT NULL DEFAULT 0
);
--Init
INSERT INTO topics (topic_name) SELECT 'main' WHERE NOT EXISTS (SELECT 1 FROM topics);
INSERT INTO stats (users, posts) SELECT 0, 0 WHERE NOT EXISTS (SELECT 1 FROM stats);
