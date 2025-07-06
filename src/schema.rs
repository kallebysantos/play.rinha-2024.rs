// @generated automatically by Diesel CLI.

diesel::table! {
    clients (id) {
        id -> Int4,
        limit -> Int4,
        balance -> Int4,
    }
}

diesel::table! {
    transactions (id) {
        id -> Int4,
        client_id -> Int4,
        value -> Int4,
        kind -> Text,
        description -> Text,
        timestamp -> Timestamptz,
    }
}

diesel::joinable!(transactions -> clients (client_id));

diesel::allow_tables_to_appear_in_same_query!(
    clients,
    transactions,
);
