use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_web::http::header::ContentType;
use apistos::{api_operation, ApiComponent};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::api::result::ResponseResult;

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct DataPath {
    val: String
}

#[derive(Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct DataRequest {
    msg: String
}

#[derive(Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct DataResponse {
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

//#[get("/data")]
#[api_operation(tag = "data", summary = "Get data")]
pub async fn get_data() -> ResponseResult<web::Json<DataResponse>> {
    let body = DataResponse {
        val: "get_data".to_string(),
        msg: "get_data".to_string()
    };
    Ok(web::Json(body))
}

//#[post("/data/{val}")]
#[api_operation(tag = "data", summary = "Post data")]
pub async fn post_data(path: web::Path<DataPath>, req: web::Json<DataRequest>) -> ResponseResult<DataResponse> {
    Ok(DataResponse {
        val: path.val.clone(),
        msg: req.msg.clone()
    })
}

#[cfg(test)]
mod tests;

