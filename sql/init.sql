CREATE TYPE role_type AS ENUM ('user', 'moderator', 'admin');
CREATE TABLE users (
  user_id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  username VARCHAR(15) NOT NULL,
  email VARCHAR(255) UNIQUE NOT NULL,
  password_hash VARCHAR NOT NULL,
  created_on TIMESTAMP NOT NULL DEFAULT (now() AT TIME ZONE 'utc'),
  last_activity TIMESTAMP NOT NULL DEFAULT (now() AT TIME ZONE 'utc'),
  user_role role_type NOT NULL DEFAULT 'user',
  token VARCHAR(24) NOT NULL --16 byte token = 24 byte base64
);
