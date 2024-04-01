// @generated automatically by Diesel CLI.

diesel::table! {
    accounts (id) {
        id -> Int4,
        sustech_id -> Int4,
        #[max_length = 30]
        name -> Varchar,
        #[max_length = 128]
        password -> Varchar,
        registered_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
