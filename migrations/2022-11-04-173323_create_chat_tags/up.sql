-- Your SQL goes here
CREATE TABLE chat_tags (
  id BIGSERIAL PRIMARY KEY,
  chat_id BIGINT references chats(id) NOT NULL,
  tag varchar(16) NOT NULL
)