use actix_http::Request;
use actix_service::Service;
use actix_web::{http::header::ContentType, test, App, Error};
use actix_web::dev::{HttpServiceFactory, ServiceResponse};
use actix_web::http::StatusCode;
use dotenv::dotenv;
use crate::api::data::routes::routes;
use crate::server::settings;
use crate::server::state::AppState;

use super::*;

pub async fn init(service_factory: impl HttpServiceFactory + 'static) -> impl Service<Request, Response = ServiceResponse, Error = Error> {
    let _ = dotenv().ok();
    let settings = settings::load("config.toml");
    test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::new(settings.clone())))
            .service(service_factory)
    ).await
}

pub async fn read_body_data<T: for<'de> Deserialize<'de>>(resp: ServiceResponse) -> T {
    let body = test::read_body(resp).await;
    serde_json::from_slice(&body).unwrap()
}

#[actix_web::test]
async fn test_get_data() {
    let app = init(routes()).await;

    let req = test::TestRequest::get().uri("/data")
        .insert_header(ContentType::json())
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let data: DataResponse = read_body_data(resp).await;
    assert_eq!(data.val, "get_data");
    assert_eq!(data.msg, "get_data");
}

#[actix_web::test]
async fn test_post_data() {
    let app = init(routes()).await;

    let req = test::TestRequest::post().uri("/data/123")
        .insert_header(ContentType::json())
        .set_json(&DataRequest { msg: "post_data".to_string() })
        .to_request();

    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);

    let data: DataResponse = read_body_data(resp).await;
    assert_eq!(data.val, "123");
    assert_eq!(data.msg, "post_data");
}
