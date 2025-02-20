use actix_web::HttpResponse;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::constants::APPLICATION_JSON;

#[derive(Debug, Deserialize, Serialize)]
pub struct Pawn {
    pub name: String,
    pub pawn_type_id: String,
    pub movement_rules: String,
    pub color: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Board {
    pub id: String,
    pub player_id_white: String,
    pub player_id_black: String,
    pub spaces: [[Option<Pawn>; 8]; 8],
    pub created_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
}

// #[derive(Debug, Deserialize, Serialize)]
// pub struct BoardRequest {
//     pub message: Option<String>,
// }

// impl BoardRequest {
//     pub fn to_Board(&self) -> Option<Board> {
//         match &self.message {
//             Some(message) => Some(Board::new(message.to_string())),
//             None => None,
//         }
//     }
// }

#[get("/board/{board_id}")]
pub async fn list() -> HttpResponse {
    // TODO: return board
    let board: Option<Board> = None;

    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(board)
}
