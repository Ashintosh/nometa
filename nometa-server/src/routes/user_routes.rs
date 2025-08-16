use actix_web::{get, post, web, HttpResponse, Responder};
use serde_json::json;

use crate::models::user_models::{AuthUser, CreateUser};
use crate::respository::user_repo::UserRepository;


#[post("/authenticate")]
async fn authenticate(user: web::Json<AuthUser>) -> impl Responder {
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
    
    match repo.create_user(user).await {
        Ok(_) => HttpResponse::Ok().json(json!({"status": "user created"})),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()}))
    }
}

pub(crate) fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
            .service(create)
            .service(authenticate),
    );
}