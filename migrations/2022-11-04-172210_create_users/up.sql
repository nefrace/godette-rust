-- Your SQL goes here

-- Your SQL goes here
CREATE TABLE users (
  id BIGINT PRIMARY KEY,
  first_name TEXT NOT NULL,
  last_name TEXT,
  username TEXT,
  karma INT DEFAULT 0,
  messages_count INT DEFAULT 0
)