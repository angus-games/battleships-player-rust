use actix_web::{get, post, HttpResponse, Responder};
use crate::services::{guess_service::GuessService, positions_service::PositionService};

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/positions")]
pub async fn positions(req_body: String) -> impl Responder {
    let response = PositionService::process_position(&req_body);
    HttpResponse::Ok().body(response)
}

#[post("/guess")]
pub async fn guess(req_body: String) -> impl Responder {
    let response = GuessService::process_guess(&req_body);
    HttpResponse::Ok().body(response)
}
