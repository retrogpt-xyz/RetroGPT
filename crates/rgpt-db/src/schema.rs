// @generated automatically by Diesel CLI.

diesel::table! {
    chats (id) {
        id -> Int4,
        head_msg -> Nullable<Int4>,
        user_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        name -> Nullable<Varchar>,
        deleted -> Bool,
    }
}

diesel::table! {
    msgs (id) {
        id -> Int4,
        body -> Text,
        sender -> Varchar,
        user_id -> Int4,
        parent_message_id -> Nullable<Int4>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    sessions (session_token) {
        session_token -> Varchar,
        user_id -> Int4,
        created_at -> Timestamp,
        expires_at -> Timestamp,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Int4,
        google_id -> Varchar,
        email -> Varchar,
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        last_login -> Timestamp,
    }
}

diesel::joinable!(chats -> msgs (head_msg));
diesel::joinable!(chats -> users (user_id));
diesel::joinable!(msgs -> users (user_id));
diesel::joinable!(sessions -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(chats, msgs, sessions, users,);
