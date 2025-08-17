use actix_web::{get, post, web, HttpResponse, Responder};
use serde_json::json;

use crate::models::user::{AuthUser, CreateUser};
use crate::repository::user::UserRepository;


#[post("/authenticate")]
async fn authenticate(user: web::Json<AuthUser>, repo: web::Data<UserRepository>) -> impl Responder {
    let user = user.into_inner();
    

    let response = json!({
        "message": format!("Received user login: {}", user.username),
        "password": user.password
    });

    HttpResponse::Ok().json(response)
}

#[post("/create")]
async fn create(user: web::Json<CreateUser>, repo: web::Data<UserRepository>) -> impl Responder {
    let user = user.into_inner();

    if let Err(e) = repo.create_user(user).await {
        if cfg!(debug_assertions) {
            return HttpResponse::InternalServerError().json(json!({"error": e.to_string()}));
        }

        return HttpResponse::InternalServerError().json(json!({"error": "Failed to create user"}))
    }

    HttpResponse::Ok().json(json!({"status": "user created"}))
}

pub(crate) fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
            .service(create)
            .service(authenticate),
    );
}