use std::sync::Mutex;
use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result};
use actix_web::http::header::ContentType;
use serde::{Deserialize, Serialize};

struct AppState {
    app_name: String,
    requests: Mutex<i32>
}

#[derive(Deserialize)]
struct DataPath {
    val: String
}

#[derive(Deserialize)]
struct DataRequest {
    msg: String
}

#[derive(Serialize)]
struct DataResponse {
    val: String,
    msg: String
}

impl Responder for DataResponse {
    type Body = actix_web::body::BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        // Create response and set content type
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }

}

#[get("/info")]
async fn get_info(data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name;
    let mut requests = data.requests.lock().unwrap();
    *requests += 1;
    HttpResponse::Ok().body(format!("name={app_name}: OK; requests: {requests}"))
}

#[post("/echo")]
async fn post_echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[get("/data")]
async fn get_data() -> Result<web::Json<DataResponse>> {
    let body = DataResponse {
        val: "get_data".to_string(),
        msg: "get_data".to_string()
    };
    Ok(web::Json(body))
}

#[post("/data/{val}")]
async fn post_data(path: web::Path<DataPath>, req: web::Json<DataRequest>) -> Result<DataResponse> {
    Ok(DataResponse {
        val: path.val.clone(),
        msg: req.msg.clone()
    })
}

fn build_state() -> web::Data<AppState> {
    web::Data::new(AppState {
        // TODO: Use config value
        app_name: String::from("Actix Web"),
        requests: Mutex::new(0)
    })
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    /*
    let v1_scope = web::scope("api/v1")
        .guard(actix_web::guard::Host("localhost:8080"))
        .service(healthcheck)
        .service(echo);
     */
    let state = build_state();

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(
                web::scope("/api")
                    .guard(actix_web::guard::Host("localhost"))
                    .service(
                        web::scope("/v1")
                            .service(get_info)
                            .service(post_echo)
                            .service(post_data)
                            .service(get_data)
                    )
            )
            .route("/healthcheck", web::get().to(HttpResponse::Ok))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
