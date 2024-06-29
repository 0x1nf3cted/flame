use actix_web::{middleware::Logger, web, App, HttpServer};
use config::db::config_db;
use env_logger::Env;
use log::{error, info};
use redis::Client;
use std::sync::{Arc, Mutex};

mod api;
mod config;
// mod middlware;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("LOGGING", "debug");
    std::env::set_var("BACKTRACE", "1");

    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

    // Initialize database pool
    let db_pool = match config_db().await {
        Ok(pool) => pool,
        Err(e) => {
            error!("Failed to configure the database: {:?}", e);
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to configure the database",
            ));
        }
    };

    eprintln!("connected to Postegres");
    let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL must be set");
    let client = Client::open(redis_url).expect("Invalid Redis URL");
    let redis_conn = Arc::new(Mutex::new(client.get_connection().unwrap()));

    info!("Starting server at http://0.0.0.0:8080");

    HttpServer::new(move || {
        let logger = Logger::default();

        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .app_data(web::Data::new(redis_conn.clone()))
            .wrap(logger)
            .service(
                web::scope("/api/v1/auth")
                    // .wrap(middlware::logger::middlware_loger)
                    .service(api::auth::login::signup)
                    .service(api::auth::confirmation::confirmation)
                    .service(api::auth::logout::logout),
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
