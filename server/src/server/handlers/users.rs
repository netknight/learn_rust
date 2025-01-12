use std::backtrace::Backtrace;
use actix_web::{get, post, web, HttpResponse};
use chrono::{DateTime, Utc};
use log::error;
use commons::errors::ServiceError;
use commons::refined::{Email, Username};
use crate::server::state::AppState;
use crate::users::domain::{User, UserEntity, UserId};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct UserData {
    pub name: String,
    pub email: String
}

impl From<User> for UserData {
    fn from(user: User) -> Self {
        Self {
            name: user.name.into_inner(),
            email: user.email.into_inner()
        }
    }
}

impl TryInto<User> for UserData {
    type Error = ServiceError;

    fn try_into(self) -> Result<User, Self::Error> {
        Ok(User {
            name: Username::try_new(self.name)
                    .map_err(|e|ServiceError::from_validation_error(e, Backtrace::capture()))?,
            email: Email::try_new(self.email)
                    .map_err(|e|ServiceError::from_validation_error(e, Backtrace::capture()))?,
        })
    }
}


#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct UserResponse {
    pub id: UserId,
    pub name: String,
    pub email: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl From<UserEntity> for UserResponse {
    fn from(entity: UserEntity) -> Self {
        Self {
            id: entity.id,
            name: entity.name.into_inner(),
            email: entity.email.into_inner(),
            created_at: entity.created_at,
            updated_at: entity.updated_at,
        }
    }
}

#[post("/users")]
async fn create_user(state: web::Data<AppState>, req: web::Json<UserData>) -> actix_web::Result<HttpResponse> {
    let user_service = &state.user_service;
    req.into_inner().try_into()
            .map_err(|e|{
                error!("Failed to parse user data: {:?}", e);
                actix_web::error::ErrorBadRequest(e)
            })
            .and_then(|user|
                    user_service.create(user)/*.await*/
                            .map_err(|e| {
                                error!("Failed to create user: {:?}", e);
                                actix_web::error::ErrorInternalServerError(e)
                            })
            )
            .map(|user|UserResponse::from(user))
            .map(|response|HttpResponse::Created().json(response))

}

#[get("/users")]
async fn get_users(state: web::Data<AppState>) -> actix_web::Result<HttpResponse> {
    let user_service = &state.user_service;

    user_service.list()
            .map_err(|e|{
                error!("Failed to list users: {:?}", e);
                actix_web::error::ErrorInternalServerError(e)
            })
            .map(|users|users.into_iter().map(UserResponse::from).collect::<Vec<UserResponse>>())
            .map(|response|HttpResponse::Ok().json(response))
}
