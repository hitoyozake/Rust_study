use actix_web::{get, App, HttpResponse, HttpServer};

#[get("/")]
async fn index() -> Result<HttpResponse, actix_web::Error> {
    let response_body = "Hello world!";

    Ok(HttpResponse::Ok().body(response_body))
}

fn main() {
    println!("Hello, world!");
}
