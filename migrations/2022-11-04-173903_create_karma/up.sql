-- Your SQL goes here
create TABLE karma (
    id BIGSERIAL PRIMARY KEY,
    chat_id BIGINT REFERENCES chats(id) NOT NULL,
    user_from BIGINT REFERENCES users(id) NOT NULL,
    user_to BIGINT REFERENCES users(id) NOT NULL,
    karma SMALLINT NOT NULL,
    message_id BIGINT NOT NULL,
    message_text TEXT,
    created TIMESTAMPTZ NOT NULL DEFAULT NOW()
)