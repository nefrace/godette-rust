// @generated automatically by Diesel CLI.

diesel::table! {
    chat_tags (id) {
        id -> Int8,
        chat_id -> Int8,
        tag -> Varchar,
    }
}

diesel::table! {
    chats (id) {
        id -> Int8,
        name -> Text,
    }
}

diesel::table! {
    karma (id) {
        id -> Int8,
        chat_id -> Int8,
        user_from -> Int8,
        user_to -> Int8,
        karma -> Int2,
        message_id -> Int8,
        message_text -> Nullable<Text>,
        created -> Timestamptz,
    }
}

diesel::table! {
    triggers (id) {
        id -> Int8,
        tag -> Varchar,
        text -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Int8,
        first_name -> Text,
        last_name -> Nullable<Text>,
        username -> Nullable<Text>,
        karma -> Nullable<Int4>,
        messages_count -> Nullable<Int4>,
    }
}

diesel::table! {
    warnings (id) {
        id -> Int8,
        chat_id -> Int8,
        user_from -> Int8,
        user_to -> Int8,
        reason -> Nullable<Text>,
        message_id -> Nullable<Int8>,
        message_text -> Nullable<Text>,
        created -> Timestamptz,
        active -> Bool,
    }
}

diesel::joinable!(chat_tags -> chats (chat_id));
diesel::joinable!(karma -> chats (chat_id));
diesel::joinable!(warnings -> chats (chat_id));

diesel::allow_tables_to_appear_in_same_query!(
    chat_tags,
    chats,
    karma,
    triggers,
    users,
    warnings,
);
