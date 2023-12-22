//! Dragon Bot Z Player collection service entry point
//!
//! Authors: Lahcène Belhadi <lahcene.belhadi@gmail.com>

mod core;
mod utils;

use crate::core::api;
use actix_web::{web, App, HttpServer};
use dbzlib_rs::database::PgDatabase;
use dbzlib_rs::util::error::Error;

#[actix_web::main]
async fn main() {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let mut pg_password = String::new();
    if let Ok(pass) = std::env::var("DBZ_PG_SUPERUSER_PASS") {
        pg_password = pass;
    } else {
        panic!("Unable to find environment variable \"DBZ_PG_SUPERUSER_PASS\"")
    }

    let database = PgDatabase::new(
        format!("postgresql://postgres:{pg_password}@dbz-player-collection-database:5434/playercollectiondb").as_str(),
    )
    .await;

    if let Err(error) = database {
        panic!("{}", Error::DatabaseConnection(error.to_string()))
    }
    let database = database.unwrap();

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(database.clone()))
            .service(api::root)
            .service(api::character_add)
    })
    .bind(("0.0.0.0", 8080));

    if let Err(error) = server {
        panic!("An error occured while binding server to ip adress and port: {error}")
    }

    // Runs the server
    let running_server = server.unwrap().run().await;
    if let Err(error) = running_server {
        panic!("An error occured while running the server: {error}")
    }
}
