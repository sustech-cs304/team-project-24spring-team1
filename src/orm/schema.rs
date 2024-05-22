// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "eventtype"))]
    pub struct Eventtype;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "role"))]
    pub struct Role;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Role;

    accounts (id) {
        id -> Int4,
        sustech_id -> Int4,
        #[max_length = 30]
        name -> Varchar,
        #[max_length = 48]
        email -> Varchar,
        #[max_length = 128]
        password -> Nullable<Varchar>,
        avatar -> Uuid,
        role -> Role,
        bio -> Text,
        registered_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    chat_members (chat_id, account_id) {
        chat_id -> Int4,
        account_id -> Int4,
        last_read -> Timestamp,
    }
}

diesel::table! {
    chat_messages (id) {
        id -> Int4,
        chat_id -> Int4,
        account_id -> Int4,
        content -> Text,
        is_deleted -> Bool,
        created_at -> Timestamp,
    }
}

diesel::table! {
    chats (id) {
        id -> Int4,
        #[max_length = 30]
        name -> Varchar,
        is_group -> Bool,
        created_at -> Timestamp,
    }
}

diesel::table! {
    comments (id) {
        id -> Int4,
        account_id -> Int4,
        event_id -> Int4,
        content -> Text,
        is_deleted -> Bool,
        created_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Eventtype;

    events (id) {
        id -> Int4,
        #[max_length = 50]
        name -> Varchar,
        kind -> Eventtype,
        description -> Text,
        organizer_id -> Int4,
        start_at -> Timestamp,
        end_at -> Timestamp,
        venue_id -> Int4,
        tickets -> Nullable<Int4>,
        registeration_deadline -> Nullable<Timestamp>,
        is_deleted -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    moment_comments (id) {
        id -> Int4,
        account_id -> Int4,
        moment_id -> Int4,
        content -> Text,
        is_deleted -> Bool,
        created_at -> Timestamp,
    }
}

diesel::table! {
    moments (id) {
        id -> Int4,
        account_id -> Int4,
        content -> Text,
        is_deleted -> Bool,
        created_at -> Timestamp,
    }
}

diesel::table! {
    participation (account_id, event_id) {
        account_id -> Int4,
        event_id -> Int4,
    }
}

diesel::table! {
    places (id) {
        id -> Int4,
        #[max_length = 30]
        name -> Varchar,
    }
}

diesel::joinable!(chat_members -> accounts (account_id));
diesel::joinable!(chat_members -> chats (chat_id));
diesel::joinable!(chat_messages -> accounts (account_id));
diesel::joinable!(chat_messages -> chats (chat_id));
diesel::joinable!(comments -> accounts (account_id));
diesel::joinable!(comments -> events (event_id));
diesel::joinable!(events -> accounts (organizer_id));
diesel::joinable!(events -> places (venue_id));
diesel::joinable!(moment_comments -> accounts (account_id));
diesel::joinable!(moment_comments -> moments (moment_id));
diesel::joinable!(moments -> accounts (account_id));
diesel::joinable!(participation -> accounts (account_id));
diesel::joinable!(participation -> events (event_id));

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    chat_members,
    chat_messages,
    chats,
    comments,
    events,
    moment_comments,
    moments,
    participation,
    places,
);
