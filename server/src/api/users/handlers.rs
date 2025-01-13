use actix_web::web;
use apistos::api_operation;
use log::error;
use crate::api::result::ResponseResult;
use crate::api::state::AppState;
use crate::api::users::models::{UserData, UserResponse, UsersResponse};

// TODO: must be Created method specified
//#[post("/")]
#[api_operation(tag = "users", summary = "Create user")]
pub async fn create_user(state: web::Data<AppState>, req: web::Json<UserData>) -> ResponseResult<web::Json<UserResponse>> {
    let user_service = &state.user_service;
    req.into_inner().try_into()
        .map_err(|e|{
            error!("Failed to parse user data: {:?}", e);
            actix_web::error::ErrorBadRequest(e)
        })
        .and_then(|user|
            user_service.create(user)/*.await?*/
                .map_err(|e| {
                    error!("Failed to create user: {:?}", e);
                    actix_web::error::ErrorInternalServerError(e)
                })
        )
        .map(|user|UserResponse::from(user))
        .map(|response|web::Json(response))

}

//#[get("/")]
#[api_operation(tag = "users", summary = "Get users")]
pub async fn get_users(state: web::Data<AppState>) -> ResponseResult<web::Json<UsersResponse>> {
    let user_service = &state.user_service;

    user_service.list()/*.await?*/
        .map_err(|e|{
            error!("Failed to list users: {:?}", e);
            actix_web::error::ErrorInternalServerError(e)
        })
        .map(|users|users.into_iter().map(UserResponse::from).collect::<Vec<UserResponse>>())
        .map(|users|UsersResponse::new(users))
        .map(|response|web::Json(response))
}
