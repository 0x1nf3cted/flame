mod api;
use api::auth::signup;
use env_logger::Env;
use log::info;
use std::sync::Arc;

use actix_web::{
    middleware::Logger,
    web::{self, Data},
    App, HttpServer,
};

use sqlx::{Pool, Postgres};
mod config;
use config::db::config_db;

// pub struct AppState {
//     pub db: Arc<Pool<Postgres>>,
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("LOGGING", "debug");
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("BACKTRACE", "1");
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let db_pool = Arc::new(config_db().await);
    info!("Starting server at http://127.0.0.1:8080");
    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .app_data(Data::new(db_pool.clone()))
            .wrap(logger)
            .service(web::scope("/api/v1/auth").service(signup::signup))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
