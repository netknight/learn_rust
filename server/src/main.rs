mod api;
mod server;
mod users;

use actix_settings::ApplySettings;
use actix_web::{middleware, App, HttpServer};
use actix_web::middleware::Logger;

use apistos::app::OpenApiWrapper;
use dotenv::dotenv;
use crate::server::{logger, settings};
use crate::api::state::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _ = dotenv().ok();
    let settings = settings::load("config.toml");
    logger::configure(&settings);

    let state = actix_web::web::Data::new(AppState::new(settings.clone()));
    log::info!("Initialisation completed.");

    HttpServer::new(move || {
        let spec = apistos::spec::Spec {
            info: apistos::info::Info {
                title: "REST API documentation".to_string(),
                description: Some(
                    "This is an API documented using Apistos,\na wonderful new tool to document your actix API !".to_string(),
                ),
                ..Default::default()
            },
            servers: vec![apistos::server::Server {
                url: "/api/v1".to_string(),
                ..Default::default()
            }],
            ..Default::default()
        };

        App::new()
            .document(spec)
            .wrap(middleware::Compress::default())
            .wrap(Logger::default())
            .app_data(state.clone())
            .service(api::routes::routes())
            .build("/openapi.json")
    })
        //.bind(("127.0.0.1", 8080))?
        .try_apply_settings(&settings)?
        .run()
        .await
}
