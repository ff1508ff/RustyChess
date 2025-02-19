#[macro_use]
extern crate actix_web;
extern crate diesel;
extern crate dotenv;

use dotenv::dotenv;
use std::{env, io};

use actix_web::{middleware, App, HttpServer};

use diesel::r2d2::ConnectionManager;
use diesel::MysqlConnection;

mod board;
mod constants;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            // register HTTP requests handlers
            .service(board::list)
    })
    .bind(("0.0.0.0", 9090))?
    .run()
    .await
}
