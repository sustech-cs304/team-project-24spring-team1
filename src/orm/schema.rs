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
        registered_at -> Timestamp,
        updated_at -> Timestamp,
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
        start_at -> Timestamp,
        end_at -> Timestamp,
        venue_id -> Int4,
        description -> Text,
        organizer_id -> Int4,
        tickets -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    participation (account, event) {
        account -> Int4,
        event -> Int4,
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
diesel::joinable!(participation -> accounts (account));
diesel::joinable!(participation -> events (event));

diesel::allow_tables_to_appear_in_same_query!(accounts, events, participation, places,);
