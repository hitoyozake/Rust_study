use actix_web::{route, web, services, App, Error, HttpResponse, HttpServer, ResponseError, Responder};
use thiserror::Error;

#[derive(Error, Debug)]
enum MyError {}


impl ResponseError for MyError{}

#[route("/", method="GET")]
async fn index() -> Result<HttpResponse, Error>{
    let response_body: &str = "Hello, world!";

    Ok(HttpResponse::Ok().body(response_body))
}

#[route("/bad", method="GET")]
async fn badrequest() -> impl Responder{
    HttpResponse::BadRequest().body("badRequest")
}

#[actix_rt::main]
async fn main() -> Result<(), actix_web::Error>{
    HttpServer::new(move || {
            App::new().service( 
                services![index, badrequest]
            )
        })
        .bind("0.0.0.0:8092")?
        .run()
        .await?;
    println!("Hello, world!");
    Ok(())
}
