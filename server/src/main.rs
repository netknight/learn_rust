mod api;
mod server;
mod users;


use actix_settings::ApplySettings;
use actix_web::{middleware, web, App, HttpServer};
use actix_web::middleware::Logger;
use dotenv::dotenv;
use crate::server::{logger, settings};
use crate::server::state::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _ = dotenv().ok();
    let settings = settings::load("config.toml");
    logger::configure(&settings);

    let state = web::Data::new(AppState::new(settings.clone()));
    log::info!("Initialisation completed.");

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Compress::default())
            .wrap(Logger::default())
            .app_data(state.clone())
            .service(
                web::scope("/api")
                    .guard(actix_web::guard::Host("localhost"))
                    .service(
                        web::scope("/v1")
                            .service(api::data::routes::routes())
                            .service(api::users::routes::routes())
                            // This service doesn't specify any new routes,
                            // must be last in the chain
                            .service(api::other::routes::routes()) 
                    )
            )
            .route("/healthcheck", web::get().to(actix_web::HttpResponse::Ok))
    })
        //.bind(("127.0.0.1", 8080))?
        .try_apply_settings(&settings)?
        .run()
        .await
}
