// @generated automatically by Diesel CLI.

diesel::table! {
    todos (id) {
        id -> Unsigned<Bigint>,
        #[max_length = 255]
        title -> Varchar,
        description -> Text,
        is_completed -> Bool,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}
