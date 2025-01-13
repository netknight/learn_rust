use actix_web::Responder;
use apistos::ApiComponent;
use serde::Serialize;
use crate::api::error::ErrorResponse;

pub type ResponseResult<T: Serialize + ApiComponent + Responder> = Result<T, ErrorResponse>; 