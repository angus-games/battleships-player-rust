use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/positions")]
async fn positions(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[post("/guess")]
async fn guess(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}


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