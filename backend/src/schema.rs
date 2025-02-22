// @generated automatically by Diesel CLI.

diesel::table! {
    game (Id) {
        Id -> Integer,
        State -> Blob,
        PlayerWhiteId -> Integer,
        PlayerBlackId -> Integer,
        CreatedAt -> Datetime,
        EndedAt -> Nullable<Datetime>,
    }
}

diesel::table! {
    pawn (Id) {
        Id -> Integer,
        #[max_length = 50]
        Name -> Varchar,
        #[max_length = 255]
        Movement -> Varchar,
    }
}

diesel::table! {
    role (Id) {
        Id -> Integer,
        #[max_length = 255]
        Name -> Varchar,
    }
}

diesel::table! {
    startPosition (Id) {
        Id -> Integer,
        State -> Blob,
    }
}

diesel::table! {
    user (Id) {
        Id -> Integer,
        RoleId -> Integer,
        #[max_length = 100]
        Username -> Varchar,
        #[max_length = 255]
        Email -> Varchar,
        CreatedAt -> Datetime,
    }
}

diesel::joinable!(user -> role (RoleId));

diesel::allow_tables_to_appear_in_same_query!(
    game,
    pawn,
    role,
    startPosition,
    user,
);
