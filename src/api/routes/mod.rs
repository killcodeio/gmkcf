use actix_web::web;
use crate::api::handlers;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/v1/mint")
            .route(web::post().to(handlers::mint_media))
    )
    .service(
        web::resource("/health")
            .route(web::get().to(handlers::health_check))
    );
}
