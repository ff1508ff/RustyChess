use super::schema::users::dsl::*;
use super::Pool;
use crate::diesel::RunQueryDsl;
use crate::schema::{roles, users};
use actix_web::{web, Error, HttpResponse};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::vec::Vec;

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Identifiable, PartialEq)]
#[diesel(table_name = roles)]
pub struct Role {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
#[diesel(table_name = users)]
#[diesel(belongs_to(Role, foreign_key = role_id))]
pub struct User {
    pub id: i32,
    pub role_id: i32,
    pub username: String,
    pub email: String,
    pub created_at: NaiveDateTime,
}

// pub fn get_users_with_roles(conn: &mut MysqlConnection) {
//     print!(
//         "{:?}",
//         user::table
//             .inner_join(role::table)
//             .load::<(User, Role)>(conn)
//     );
// }

#[get("/users")]
pub async fn get_users(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || get_all_users(db))
        .await
        .map(|user| HttpResponse::Ok().json(user.unwrap()))
        .map_err(|_| actix_web::error::ErrorInternalServerError("Database error"))?)
}

fn get_all_users(pool: web::Data<Pool>) -> Result<Vec<User>, diesel::result::Error> {
    let conn = &mut pool.get().unwrap();
    let items = users.load::<User>(conn)?;
    Ok(items)
}
