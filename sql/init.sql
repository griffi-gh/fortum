DROP TABLE IF EXISTS topics CASCADE;
DROP TABLE IF EXISTS posts CASCADE;
DROP TABLE IF EXISTS votes CASCADE;
DROP TABLE IF EXISTS users CASCADE;
DROP TYPE IF EXISTS role_type; 

CREATE TYPE role_type AS ENUM ('banned', 'unverified', 'user', 'moderator', 'admin');
CREATE TABLE users (
  user_id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  username VARCHAR(15) NOT NULL,
  email VARCHAR(255) UNIQUE NOT NULL,
  password_hash VARCHAR NOT NULL,
  created_on TIMESTAMPTZ NOT NULL DEFAULT now(),
  last_activity TIMESTAMPTZ NOT NULL DEFAULT now(),
  user_role role_type NOT NULL DEFAULT 'user',
  token VARCHAR(24) UNIQUE NOT NULL CHECK (length(token) = 24), --16 byte token = 24 byte base64
  profile_image VARCHAR,
  banner_image VARCHAR
);
CREATE TABLE topics (
  topic_id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  topic_name VARCHAR(30) UNIQUE NOT NULL,
  created_on TIMESTAMPTZ NOT NULL DEFAULT now(),
  creator INTEGER,
  FOREIGN KEY(creator) 
    REFERENCES users(user_id)
    ON DELETE SET NULL
);
CREATE TABLE posts (
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
CREATE TABLE votes (
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
INSERT INTO topics (topic_name) VALUES('main');
