use actix_web::cookie::{Cookie, SameSite};
use actix_web::{get, post, web, HttpResponse, HttpRequest, Responder};
use serde_json::json;
use sqlx::{PgPool, Pool, Postgres};

use crate::models::user::{AuthUser, CreateUser};
use crate::repository::user::UserRepository;
use crate::utils::redis::RedisPool;


#[post("/")]
async fn get_user(req: HttpRequest, db_pool: web::Data<Pool<Postgres>>, redis_pool: web::Data<RedisPool>) -> impl Responder {
    let session_cookie = match req.cookie("session_id") {
        Some(c) => c.value().to_string(),
        None => return HttpResponse::Unauthorized().json(json!({"status": "err", "message": "unauthorized"}))
    };

    let user_id = match redis_pool.get_session(&session_cookie).await {
        Ok(Some(id)) => id,
        _ => return HttpResponse::Unauthorized().json(json!({"status": "err", "message": "unauthorized"}))
    };

    let db_pool = db_pool.into_inner().as_ref().clone();
    let repo = UserRepository::new(db_pool);

    match repo.get_user_by_id(user_id).await {
        Ok(user_data) => HttpResponse::Ok().json(json!(user_data)),
        Err(_) => HttpResponse::InternalServerError().json(json!({"status": "err", "message": "Failed to fetch user"}))
    }
}

#[post("/authenticate")]
async fn authenticate(user: web::Json<AuthUser>, db_pool: web::Data<Pool<Postgres>>, redis_pool: web::Data<RedisPool>) -> impl Responder {
    let user = user.into_inner();
    let db_pool = db_pool.into_inner().as_ref().clone();
    let repo = UserRepository::new(db_pool);

    match repo.authenticate_user(user).await {
        Ok(user_id) => {
            // store session in memcached with expiry -> user_data.id
            let session_id = uuid::Uuid::new_v4().to_string();

            // 24 hours = 86400 seconds
            if let Ok(()) = redis_pool.set_session(&session_id, user_id, 86400).await {
                let cookie = Cookie::build("session_id", session_id)
                    .path("/")
                    .http_only(true)
                    .secure(true)
                    .same_site(SameSite::None)
                    .finish();

                return HttpResponse::Ok().cookie(cookie).json(json!({"status": "ok"}));
            }
            
            HttpResponse::InternalServerError().json(json!({"status": "err", "message": "redis-set-error"}))
        },
        Err(e) => HttpResponse::Unauthorized().json(json!({"status": "err", "msg": "unauthorized"}))
    }
}

#[post("/create")]
async fn create(user: web::Json<CreateUser>, pool: web::Data<PgPool>) -> impl Responder {
    let user = user.into_inner();
    let pool = pool.into_inner().as_ref().clone();
    let repo = UserRepository::new(pool);

    match repo.create_user(user).await {
        Ok(()) => HttpResponse::Ok().json(json!({"status": "ok"})),
        Err(e) => HttpResponse::InternalServerError().json(json!({"status": "err", "message": "internal-server-error"}))
    }
}

pub(crate) fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
            .service(create)
            .service(authenticate),
    );
}