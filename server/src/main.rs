mod api;
mod server;
mod users;

use actix_settings::ApplySettings;
use actix_web::{get, middleware, post, web, App, HttpResponse, HttpServer, Responder, ResponseError};
use actix_web::middleware::Logger;
use apistos::actix::CreatedJson;
use apistos::info::Info;
use apistos::server::Server;
use apistos::spec::Spec;
use dotenv::dotenv;
use apistos::app::OpenApiWrapper;
use crate::api::error::ErrorResponse;
use crate::api::users::models::UserData;
use crate::server::{logger, settings};
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


#[apistos::api_operation(tag = "users", summary = "Add a new pet to the store")]
async fn get_test(body: web::Json<UserData>) -> Result<CreatedJson<UserData>, ErrorResponse> {
    Ok(CreatedJson(body.0))
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
        let spec = Spec {
            info: Info {
                title: "REST API documentation".to_string(),
                description: Some(
                    "This is an API documented using Apistos,\na wonderful new tool to document your actix API !".to_string(),
                ),
                ..Default::default()
            },
            servers: vec![Server {
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
            .service(
                apistos::web::scope("/dev").service(
                    apistos::web::resource("/test").route(apistos::web::get().to(get_test))
                )
                
            )
            .route("/healthcheck", apistos::web::get().to(get_test))
            .build("./openapi.json")
    })
        //.bind(("127.0.0.1", 8080))?
        .try_apply_settings(&settings)?
        .run()
        .await
}
