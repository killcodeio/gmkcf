use actix_web::{App, HttpServer, middleware};
use env_logger;
use log::info;

mod api;
mod core;
mod domain;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 1. Init Logging
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    info!("Starting GMKCF (Genesis Media Key Creator/Factory) on port 8080");

    // 2. Start Server
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .configure(api::routes::configure_routes)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
