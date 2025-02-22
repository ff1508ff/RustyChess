use super::schema::role::dsl::*;
use super::schema::user::dsl::*;
use super::Pool;
use crate::diesel::RunQueryDsl;
use actix_web::{web, Error, HttpResponse};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::vec::Vec;

#[derive(Debug, Serialize, Deserialize, Queryable)]
#[diesel(table_name = role)]
pub struct Role {
    #[diesel(column_name = Id)]
    pub id: i32,
    #[diesel(column_name = Name)]
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Associations)]
#[diesel(belongs_to(Role, foreign_key = RoleId))]
#[diesel(table_name = user)]
pub struct User {
    #[diesel(column_name = Id)]
    pub id: i32,
    #[diesel(column_name = RoleId)]
    pub role_id: i32,
    #[diesel(column_name = Username)]
    pub username: String,
    #[diesel(column_name = Email)]
    pub email: String,
    #[diesel(column_name = CreatedAt)]
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
    let db = db.clone();
    let users = web::block(move || get_all_users(db))
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Database query failed"))?;

    Ok(HttpResponse::Ok().json(users))
}

fn get_all_users(pool: web::Data<Pool>) -> Result<Vec<User>, diesel::result::Error> {
    let conn = pool.get().map_err(|_| diesel::result::Error::NotFound)?;
    let items = user.load::<User>(&conn)?;
    Ok(items)
}
