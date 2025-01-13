use actix_web::Scope;
use actix_web::web::{get, post, scope};
use super::handlers;

pub fn routes() -> Scope {
    scope("/data")
        .route("", get().to(handlers::get_data))
        .route("/{val}", post().to(handlers::post_data))
}