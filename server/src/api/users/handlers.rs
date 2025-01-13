use actix_web::{get, post, web};
use actix_web::web::Json;
use apistos::actix::CreatedJson;
use apistos::api_operation;
use crate::api::error::ErrorResponse;
use crate::api::result::ResponseResult;
use crate::api::users::models::{UserData, UserResponse, UsersResponse};
use crate::server::state::AppState;

#[post("/users")]
#[api_operation(summary = "Create user")]
async fn create_user(state: web::Data<AppState>, req: Json<UserData>) -> ResponseResult<CreatedJson<UserResponse>> {
    let user_service = &state.user_service;
    let user = req.into_inner().into();
    let result = user_service.create(user)?/*.await?*/;
    Ok(CreatedJson(result.into()))
}

#[get("/users")]
#[api_operation(summary = "Get users")]
async fn get_users(state: web::Data<AppState>) -> Result<Json<UsersResponse>, ErrorResponse> {
    let user_service = &state.user_service;
    let users = user_service.list()?/*.await?*/;
    Ok(Json(users.into()))
}
