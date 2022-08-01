CREATE TABLE IF NOT EXISTS users (
  user_id serial PRIMARY KEY,
  username VARCHAR(15) UNIQUE NOT NULL,
  email VARCHAR(255) UNIQUE NOT NULL,
  password_hash VARCHAR NOT NULL,
  created_on TIMESTAMP NOT NULL,
  last_activity TIMESTAMP NOT NULL
);
