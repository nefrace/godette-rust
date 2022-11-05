-- Your SQL goes here
create TABLE warnings (
    id BIGSERIAL PRIMARY KEY,
    chat_id BIGINT REFERENCES chats(id) NOT NULL,
    user_from BIGINT REFERENCES users(id) NOT NULL,
    user_to BIGINT REFERENCES users(id) NOT NULL,
    reason TEXT,
    message_id BIGINT,
    message_text TEXT,
    created TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    active BOOLEAN NOT NULL DEFAULT TRUE
)