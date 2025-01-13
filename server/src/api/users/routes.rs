use apistos::web::{delete, get, post, put, resource, scope, Scope};
use crate::api::users::handlers::get_users;

pub(crate) fn routes() -> Scope {
    scope("/users")
        .service(resource("/").route(get().to(get_users)))
}