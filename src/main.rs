use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use ethers_core::types::{U256};
use serde::Deserialize;

#[derive(Deserialize)]
struct VerifySignature {
    pub r: U256,
    /// S Value
    pub s: U256,
    /// V value
    pub v: u64,
    address: String
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(verify_signature)
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[get("/verify_signature")]                       // This get request should pass in the signature, hash and parameters in the URL
async fn verify_signature(info: web::Query<VerifySignature>) -> impl Responder {  // Should return a boolean
    println!("{} {} {} {}", info.address, info.v, info.r, info.s);
    HttpResponse::Ok().body("Hello verify!")      // Message can be stored in server because expecting it anyways
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}