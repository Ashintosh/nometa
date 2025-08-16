mod routes;
mod models;
mod respository;
mod utils;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use actix_web::middleware::NormalizePath;
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(&db_url)
        .await
        .expect("Failed to create DB pool");

    let user_repo = web::Data::new(respository::user_repo::UserRepository::new(pool));

    println!("Server running at http://0.0.0.0:8080");
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors)
            .wrap(NormalizePath::default())
            .service(routes::index)
            .service(routes::health)
            .configure(routes::user_routes::config)
            .app_data(user_repo.clone())
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
