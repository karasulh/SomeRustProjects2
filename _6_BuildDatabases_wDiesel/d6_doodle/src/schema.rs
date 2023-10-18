// @generated automatically by Diesel CLI.

diesel::table! {
    polls (id) {
        id -> Int4,
        question -> Varchar,
        options -> Varchar,
        owner -> Nullable<Int4>,
    }
}

diesel::table! {
    responses (poll_id, user_id) {
        poll_id -> Int4,
        user_id -> Int4,
        selected -> Nullable<Int4>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        password -> Varchar,
    }
}

diesel::joinable!(polls -> users (owner));
diesel::joinable!(responses -> polls (poll_id));
diesel::joinable!(responses -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    polls,
    responses,
    users,
);
