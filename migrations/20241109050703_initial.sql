-- Add migration script here
-- create user table
CREATE TABLE IF NOT EXISTS users(
    id bigserial PRIMARY KEY,
    ws_id bigint NOT NULL,
    fullname varchar(64) NOT NULL,
    email varchar(64) NOT NULL,
    password_hash varchar(97) NOT NULL,
    created_at timestamptz DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS user_email_idx ON users(email);

CREATE TABLE IF NOT EXISTS workspaces(
  id bigserial PRIMARY KEY,
  name varchar(32) NOT NULL UNIQUE,
  owner_id bigint NOT NULL,
  created_at timestamptz DEFAULT CURRENT_TIMESTAMP
);

-- create chat type: single, group, private_channel, public_channel
CREATE TYPE chat_type AS ENUM(
  'single',
  'group',
  'private_channel',
  'public_channel'
);

-- create chat table
CREATE TABLE IF NOT EXISTS chats(
  id bigserial PRIMARY KEY,
  ws_id bigint NOT NULL,
  name varchar(64),
  chat_type chat_type NOT NULL,
  -- user id list
  members bigint[] NOT NULL,
  status SMALLINT NOT NULL DEFAULT 1,
  created_at timestamptz DEFAULT CURRENT_TIMESTAMP,
  UNIQUE (ws_id, name, members)
);

-- create message table
CREATE TABLE IF NOT EXISTS messages(
  id bigserial PRIMARY KEY,
  chat_id bigint NOT NULL,
  sender_id bigint NOT NULL,
  content text NOT NULL,
  created_at timestamptz DEFAULT CURRENT_TIMESTAMP
);
