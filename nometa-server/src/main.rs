mod routes;
mod models;
mod repository;
mod utils;
mod errors;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use actix_web::middleware::NormalizePath;
use flexi_logger::{Age, Cleanup, Criterion, FileSpec, Logger, Naming, WriteMode};
use sqlx::{Pool, Postgres};

use crate::utils::database::{PostgreSQL, PostgreSQLConfig};
use crate::utils::redis::{RedisConfig, RedisPool};


#[actix_web::main]
async fn main() -> Result<(), String> {
    start_logger().map_err(|e| {
        if cfg!(debug_assertions) {
            return e.to_string();
        }
    
        "Failed to start logger".into()
    })?;

    start_http_server("0.0.0.0", 8080).await
}

async fn start_http_server(http_host: &str, http_port: u16) -> Result<(), String> {
    let db_pool = get_postgres_pool().await?;
    let redis_pool = get_redis_pool()?;

    println!("Server running at http://{}:{}", http_host, http_port);
    HttpServer::new(move || {
        // TODO: Add allowed origins for production builds
        #[cfg(not(debug_assertions))]
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![actix_web::http::header::AUTHORIZATION, actix_web::http::header::CONTENT_TYPE])
            .supports_credentials();

        #[cfg(debug_assertions)]
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
            .app_data(web::Data::new(db_pool.clone()))
            .app_data(web::Data::new(redis_pool.clone()))
    })
    .bind((http_host, http_port))
    .map_err(|e| {
        if cfg!(debug_assertions) {
            return e.to_string();
        }
        format!("Failed to bind {} on port {}", http_host, http_port)
    })?
    .run()
    .await
    .map_err(|e| {
        if cfg!(debug_assertions) {
            return e.to_string();
        }
        "Could not start HTTP server".into()
    })?;

    Ok(())
}

fn start_logger() -> Result<(), String> {
    Logger::try_with_str("info")
        .map_err(|e| {
            if cfg!(debug_assertions) {
                return e.to_string();
            }
            "Failed to initialize logger".into()
        })?
        .log_to_file(
            FileSpec::default()
                .directory("logs")
                .basename("nometa")
                .suffix("log")
        )
        .write_mode(WriteMode::BufferAndFlush)
        .rotate(
            Criterion::Age(Age::Day),
            Naming::Timestamps,
            Cleanup::KeepLogFiles(10)
        )
        .start()
        .map_err(|e| {
            if cfg!(debug_assertions) {
                return e.to_string();
            }
            "Failed to start logger".into()
        })?;

    Ok(())
}

async fn get_postgres_pool() -> Result<Pool<Postgres>, String> {
    // Create Postgres pool
    let db_config = PostgreSQLConfig::from_env()
        .map_err(|e| {
            if cfg!(debug_assertions) {
                return e.to_string();
            }
            "Failed to get required ENV variables for Postgres".into()
        })?;

    let db_pool = PostgreSQL::new(db_config)
        .await
        .map_err(|e| {
            if cfg!(debug_assertions) {
                return e.to_string();
            }
            "Failed to create PostgreSQL pool".into()
        })?
        .pool();

    Ok(db_pool)
}

fn get_redis_pool() -> Result<RedisPool, String> {
    // Create Redis pool
    let redis_config = RedisConfig::from_env()
        .map_err(|e| {
            if cfg!(debug_assertions) {
                return e.to_string();
            }
            "Failed to get required ENV variables for Redis".into()
        })?;

    RedisPool::new(redis_config)
        .map_err(|e| {
            if cfg!(debug_assertions) {
                return e.to_string();
            }
            "Failed to create Redis pool".into()
        })
}


