CREATE TYPE role_type AS ENUM ('user', 'moderator', 'admin');
CREATE TABLE users (
  user_id serial PRIMARY KEY,
  username VARCHAR(15) NOT NULL,
  email VARCHAR(255) UNIQUE NOT NULL,
  password_hash VARCHAR NOT NULL,
  created_on TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  last_activity TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  user_role role_type NOT NULL DEFAULT 'user'
);
CREATE TABLE tokens (
  token VARCHAR PRIMARY KEY,
  user_id integer NOT NULL,
  expiry TIMESTAMP NOT NULL DEFAULT now() + INTERVAL '1 year 3 hours 20 minutes',
  CONSTRAINT FOREIGN KEY(user_id)
    REFERENCES users(user_id)
    ON DELETE CASCADE
);
