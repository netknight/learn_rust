use apistos::web::{get, post, scope, Scope};
use super::handlers;

pub fn routes() -> Scope {
    scope("/users")
        .route("", get().to(handlers::get_users))
        .route("", post().to(handlers::create_user))
}