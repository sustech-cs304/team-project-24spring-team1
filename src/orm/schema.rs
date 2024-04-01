// @generated automatically by Diesel CLI.

pub mod sql_types {
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
