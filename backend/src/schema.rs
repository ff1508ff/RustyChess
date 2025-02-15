// @generated automatically by Diesel CLI.

diesel::table! {
    Games (id) {
        id -> Integer,
        state -> Blob,
    }
}

diesel::table! {
    MovementPieces (id) {
        id -> Integer,
        movementsId -> Integer,
        piescesId -> Integer,
    }
}

diesel::table! {
    Movements (id) {
        id -> Integer,
        rule -> Blob,
    }
}

diesel::table! {
    Pieces (id) {
        id -> Integer,
        #[max_length = 50]
        name -> Varchar,
    }
}

diesel::table! {
    StartPositions (id) {
        id -> Integer,
        state -> Blob,
    }
}

diesel::joinable!(MovementPieces -> Movements (movementsId));
diesel::joinable!(MovementPieces -> Pieces (piescesId));

diesel::allow_tables_to_appear_in_same_query!(
    Games,
    MovementPieces,
    Movements,
    Pieces,
    StartPositions,
);
