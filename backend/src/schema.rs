// @generated automatically by Diesel CLI.

diesel::table! {
    Game (Id) {
        Id -> Integer,
        State -> Blob,
        PlayerWhiteId -> Integer,
        PlayerBlackId -> Integer,
        CreatedAt -> Datetime,
        EndedAt -> Nullable<Datetime>,
    }
}

diesel::table! {
    Pawn (Id) {
        Id -> Integer,
        #[max_length = 50]
        Name -> Varchar,
        #[max_length = 255]
        Movement -> Varchar,
    }
}

diesel::table! {
    Role (Id) {
        Id -> Integer,
        #[max_length = 255]
        Name -> Varchar,
    }
}

diesel::table! {
    StartPosition (Id) {
        Id -> Integer,
        State -> Blob,
    }
}

diesel::table! {
    User (Id) {
        Id -> Integer,
        RoleId -> Integer,
        #[max_length = 100]
        Username -> Varchar,
        #[max_length = 255]
        Email -> Varchar,
        CreatedAt -> Datetime,
    }
}

diesel::joinable!(User -> Role (RoleId));

diesel::allow_tables_to_appear_in_same_query!(
    Game,
    Pawn,
    Role,
    StartPosition,
    User,
);
