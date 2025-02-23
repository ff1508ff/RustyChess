// @generated automatically by Diesel CLI.

diesel::table! {
    games (id) {
        id -> Integer,
        state -> Blob,
        player_id_white -> Integer,
        player_id_black -> Integer,
        created_at -> Datetime,
        ended_at -> Nullable<Datetime>,
    }
}

diesel::table! {
    pawns (id) {
        id -> Integer,
        #[max_length = 50]
        name -> Varchar,
        #[max_length = 255]
        movement -> Varchar,
    }
}

diesel::table! {
    roles (id) {
        id -> Integer,
        #[max_length = 255]
        name -> Varchar,
    }
}

diesel::table! {
    startPositions (id) {
        id -> Integer,
        state -> Blob,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        role_id -> Integer,
        #[max_length = 100]
        username -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        created_at -> Datetime,
    }
}

diesel::joinable!(users -> roles (role_id));

diesel::allow_tables_to_appear_in_same_query!(
    games,
    pawns,
    roles,
    startPositions,
    users,
);
