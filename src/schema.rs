use diesel::prelude::*;

diesel::table! {
    clients(id) {
        id -> Integer,
        limit -> Integer,
        balance -> Integer,
    }

}

diesel::table! {
    transactions(id) {
        id -> Integer,
        client_id -> Integer,
        value -> Integer,
        kind -> Text,
        description -> Text,
        timestamp -> Nullable<Timestamptz>,
    }
}

joinable!(transactions -> clients(client_id));
allow_tables_to_appear_in_same_query!(clients, transactions);
