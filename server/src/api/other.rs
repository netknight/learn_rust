pub(crate) mod handlers {
    use actix_web::web;
    use apistos::actix::NoContent;
    use apistos::api_operation;
    use crate::api::result::ResponseResult;
    use crate::api::state::AppState;

    #[api_operation(tag = "system", summary = "Healthcheck")]
    pub async fn healthcheck() -> ResponseResult<NoContent> {
        Ok(NoContent)
    }
    
    //#[get("/info")]
    #[api_operation(tag = "other", summary = "Get service info")]
    pub async fn get_info(state: web::Data<AppState>) -> ResponseResult<String> {
        let app_name = &state.settings.application.name;
        let mut requests = state.requests.lock().unwrap();
        *requests += 1;
        Ok(format!("name={app_name}: OK; requests: {requests}"))
    }

    //#[post("/echo")]
    #[api_operation(tag = "other", summary = "Echo request body")]
    pub async fn post_echo(req_body: String) -> ResponseResult<String> {
        Ok(req_body)
    }
}

pub(crate) mod routes {
    use apistos::web::{get, post, scope, Scope};
    use super::handlers;

    pub fn routes() -> Scope {
        scope("")
            .route("/info", get().to(handlers::get_info))
            .route("/echo", post().to(handlers::post_echo))
    }
}
