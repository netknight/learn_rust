use actix_web::Scope;
use actix_web::web::{get, post, scope};
use super::handlers;

pub fn routes() -> Scope {
    scope("/users")
        .route("", get().to(handlers::get_users))
        .route("", post().to(handlers::create_user))
}