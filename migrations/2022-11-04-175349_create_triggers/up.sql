-- Your SQL goes here

CREATE TABLE triggers (
    id BIGSERIAL PRIMARY KEY,
    tag VARCHAR(16) NOT NULL,
    text TEXT NOT NULL
)