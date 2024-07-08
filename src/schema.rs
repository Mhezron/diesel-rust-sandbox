// @generated automatically by Diesel CLI.

diesel::table! {
    loans (id) {
        id -> Int4,
        user_id -> Int4,
        amount -> Float8,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
    }
}

diesel::joinable!(loans -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    loans,
    users,
);
