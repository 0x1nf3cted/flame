mod api;

use actix_web::{middleware::Logger, post, App, web, HttpResponse, HttpServer, Responder};

use api::auth::auth;





#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("LOGGING", "debug");
    std::env::set_var("BACKTRACE", "1");
    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .service(web::scope("/api/v1/auth").service(auth::signup))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
