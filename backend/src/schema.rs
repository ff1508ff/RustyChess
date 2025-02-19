// @generated automatically by Diesel CLI.

diesel::table! {
    Games (Id) {
        Id -> Integer,
        State -> Blob,
    }
}

diesel::table! {
    Pieces (Id) {
        Id -> Integer,
        #[max_length = 50]
        Name -> Varchar,
        #[max_length = 255]
        Movement -> Varchar,
    }
}

diesel::table! {
    StartPositions (Id) {
        Id -> Integer,
        State -> Blob,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    Games,
    Pieces,
    StartPositions,
);
