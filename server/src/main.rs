mod server;
mod users;

use actix_settings::ApplySettings;
use actix_web::{get, middleware, post, web, App, HttpResponse, HttpServer, Responder};
use actix_web::middleware::Logger;
use dotenv::dotenv;
use crate::server::{logger, settings};
use crate::server::handlers;
use crate::server::state::AppState;

#[get("/info")]
async fn get_info(state: web::Data<AppState>) -> impl Responder {
    let app_name = &state.settings.application.name;
    let mut requests = state.requests.lock().unwrap();
    *requests += 1;
    HttpResponse::Ok().body(format!("name={app_name}: OK; requests: {requests}"))
}

#[post("/echo")]
async fn post_echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    /*
    let v1_scope = web::scope("api/v1")
        .guard(actix_web::guard::Host("localhost:8080"))
        .service(healthcheck)
        .service(echo);
     */
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
                            .service(get_info)
                            .service(post_echo)
                            .service(handlers::data::post_data)
                            .service(handlers::data::get_data)
                            .service(handlers::users::create_user)
                            .service(handlers::users::get_users)
                    )
            )
            .route("/healthcheck", web::get().to(HttpResponse::Ok))
    })
        //.bind(("127.0.0.1", 8080))?
        .try_apply_settings(&settings)?
        .run()
        .await
}
