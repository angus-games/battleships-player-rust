mod routes;
mod services;
mod models;

use actix_web::{App, HttpServer};
use crate::routes::{hello, guess, positions};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(positions)
            .service(guess)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
