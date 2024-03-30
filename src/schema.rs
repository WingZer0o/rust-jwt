// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 50]
        email -> Varchar,
        #[max_length = 150]
        hash -> Varchar,
        created_at -> Timestamp,
    }
}
