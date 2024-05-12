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
        #[max_length = 128]
        password -> Varchar,
        role -> Role,
        bio -> Text,
        registered_at -> Timestamp,
        updated_at -> Timestamp,
        avatar -> Uuid,
        #[max_length = 48]
        email -> Varchar,
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
        created_at -> Timestamp,
        updated_at -> Timestamp,
        is_deleted -> Bool,
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

diesel::joinable!(events -> accounts (organizer_id));
diesel::joinable!(events -> places (venue_id));
diesel::joinable!(participation -> accounts (account_id));
diesel::joinable!(participation -> events (event_id));

diesel::allow_tables_to_appear_in_same_query!(accounts, events, participation, places,);
