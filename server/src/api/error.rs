use std::fmt::{Display, Formatter};
use actix_http::StatusCode;
use actix_web::ResponseError;
use serde;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, apistos::ApiErrorComponent)]
#[openapi_error(
    status(code = 400),
    status(code = 401),
    status(code = 403),
    status(code = 404),
    status(code = 405, description = "Invalid input"),
    status(code = 409),
    status(code = 500)
)]
pub enum ErrorResponse {
    BadRequest(String),
    Unauthorized(String),
    Forbidden(String),
    NotFound(String),
    MethodNotAllowed(String),
    Conflict(String),
    InternalServerError(String)
}

impl Display for ErrorResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        /*
        let type_name = std::any::type_name::<Self>()
            .split("::").last().unwrap_or("ErrorResponse[unknown]");
        write!(f, "{type_name}")
        */
        write!(f, "{:?}", self)

    }
}

impl ResponseError for ErrorResponse {
    fn status_code(&self) -> StatusCode {
        match self {
            ErrorResponse::BadRequest(_) => StatusCode::BAD_REQUEST,
            ErrorResponse::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            ErrorResponse::Forbidden(_) => StatusCode::FORBIDDEN,
            ErrorResponse::NotFound(_) => StatusCode::NOT_FOUND,
            ErrorResponse::MethodNotAllowed(_) => StatusCode::METHOD_NOT_ALLOWED,
            ErrorResponse::Conflict(_) => StatusCode::CONFLICT,
            ErrorResponse::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

// TODO: Implement specific error types for correct error types transformation
impl From<Box<dyn std::error::Error>> for ErrorResponse {
    fn from(err: Box<dyn std::error::Error>) -> Self {
        ErrorResponse::InternalServerError(err.to_string())
    }
}