pub(crate) mod handlers {
    use actix_web::{web, HttpResponse, Responder};
    use crate::server::state::AppState;
    
    //#[get("/info")]
    pub async fn get_info(state: web::Data<AppState>) -> impl Responder {
        let app_name = &state.settings.application.name;
        let mut requests = state.requests.lock().unwrap();
        *requests += 1;
        HttpResponse::Ok().body(format!("name={app_name}: OK; requests: {requests}"))
    }

    //#[post("/echo")]
    pub async fn post_echo(req_body: String) -> impl Responder {
        HttpResponse::Ok().body(req_body)
    }
}

pub(crate) mod routes {
    use actix_web::Scope;
    use actix_web::web::{get, post, scope};
    use super::handlers;

    pub fn routes() -> Scope {
        scope("")
            .route("/info", get().to(handlers::get_info))
            .route("/echo", post().to(handlers::post_echo))
    }
}