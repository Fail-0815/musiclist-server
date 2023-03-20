// @generated automatically by Diesel CLI.

diesel::table! {
    sessions (id) {
        id -> Uuid,
    }
}

diesel::table! {
    votes (wishid, sessionid) {
        wishid -> Int4,
        sessionid -> Uuid,
    }
}

diesel::table! {
    wishes (id) {
        id -> Int4,
        title -> Varchar,
        artist -> Varchar,
        comment -> Text,
        score -> Int2,
        sessionid -> Uuid,
    }
}

diesel::joinable!(votes -> sessions (sessionid));
diesel::joinable!(votes -> wishes (wishid));

diesel::allow_tables_to_appear_in_same_query!(
    sessions,
    votes,
    wishes,
);
