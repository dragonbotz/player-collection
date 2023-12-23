//! Service's API
//!
//! This module contains the API implementation of the service
//!
//! Authors: Lahcène Belhadi <lahcene.belhadi@gmail.com>
use crate::utils::model::payload::PayloadCharacterAdd;
use actix_web::{get, post, web, HttpResponse, Responder};
use dbzlib_rs::database::PgDatabase;

#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok().body("Hello and welcome to Dragon Bot Z's player collection service!")
}

/// Inserts a character to the player's collection
///
/// # Arguments
/// * player - the player id
/// * character - the character id
///
/// # Returns
/// Returns a 200 HTTP code if everything is alright
#[post("/character/add")]
async fn character_add(
    database: web::Data<PgDatabase>,
    payload: web::Json<PayloadCharacterAdd>,
) -> impl Responder {
    let pool = database.pool();
    let payload: PayloadCharacterAdd = payload.into_inner();

    let insert_result = sqlx::query("INSERT INTO character (player, character) VALUES ($1, $2)")
        .bind(payload.player)
        .bind(payload.character)
        .execute(pool)
        .await;

    if let Err(_) = insert_result {
        return HttpResponse::InternalServerError()
    }

    HttpResponse::Ok()
}
