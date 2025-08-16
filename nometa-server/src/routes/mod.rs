pub(super) mod user_routes;

use actix_web::{web, get, HttpResponse, Responder};
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
struct ApiResponse {
    message: String,
}

#[get("/")]
async fn index() -> impl Responder {
    let response = ApiResponse {
        message: "Hello from index".to_string(),
    };
    HttpResponse::Ok().json(response)
}

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("OK")
}
