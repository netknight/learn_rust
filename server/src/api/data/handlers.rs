use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use actix_web::http::header::ContentType;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct DataPath {
    val: String
}

#[derive(Serialize, Deserialize)]
pub struct DataRequest {
    msg: String
}

#[derive(Serialize, Deserialize)]
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
pub async fn get_data() -> actix_web::Result<web::Json<DataResponse>> {
    let body = DataResponse {
        val: "get_data".to_string(),
        msg: "get_data".to_string()
    };
    Ok(web::Json(body))
}

//#[post("/data/{val}")]
pub async fn post_data(path: web::Path<DataPath>, req: web::Json<DataRequest>) -> actix_web::Result<DataResponse> {
    Ok(DataResponse {
        val: path.val.clone(),
        msg: req.msg.clone()
    })
}

#[cfg(test)]
mod tests;

