use actix_web::Responder;
use apistos::ApiComponent;
use serde::Serialize;

pub type ResponseResult<T: Serialize + ApiComponent + Responder> = Result<T, actix_web::Error>;