mod routes;
mod models;
mod repository;
mod utils;
mod errors;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use actix_web::middleware::NormalizePath;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_config = utils::database::PostgreSQLConfig::from_env().expect("Failed to get ENV");
    let pool = utils::database::PostgreSQL::new(db_config).await.expect("Failed to create DB pool").pool();
    let user_repo = web::Data::new(repository::user::UserRepository::new(pool));

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
            .configure(routes::user::config)
            .app_data(user_repo.clone())
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}


