use apistos::web::{get, scope, Scope};

pub fn routes() -> Scope {
    scope("/api")
        .guard(actix_web::guard::Host("localhost"))
        .service(
            scope("/v1")
                .service(super::data::routes::routes())
                .service(super::users::routes::routes())
                // This service doesn't specify any new routes,
                // must be last in the chain
                .service(super::other::routes::routes())
        )
        .route("/healthcheck", get().to(super::other::handlers::healthcheck)
    )
}