use super::handlers;
use apistos::web::{get, post, scope, Scope};

pub fn routes() -> Scope {
    scope("/data")
        .route("", get().to(handlers::get_data))
        .route("/{val}", post().to(handlers::post_data))
}
