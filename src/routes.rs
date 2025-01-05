use actix_web::{get, post, web, HttpResponse, Responder};
use crate::models::game_state::GameState;
use crate::services::{guess_service::GuessService, positions_service::PositionService};


#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/positions")]
pub async fn positions(payload: web::Json<GameState>) -> impl Responder {
    let response = PositionService::process_position(&payload);
    HttpResponse::Ok().body(response)
}

#[post("/guess")]
pub async fn guess(payload: web::Json<GameState>) -> impl Responder {
    let response = GuessService::process_guess(&payload);
    HttpResponse::Ok().body(response)
}
