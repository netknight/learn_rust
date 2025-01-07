use actix_web::{get, post, web, HttpResponse};
use chrono::{DateTime, Utc};
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
            name: user.name,
            email: user.email
        }
    }
}

impl Into<User> for UserData {
    fn into(self) -> User {
        User::new(self.name, self.email).unwrap() // TODO: Handle error
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
            name: entity.name,
            email: entity.email,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
        }
    }
}

#[post("/users")]
async fn create_user(state: web::Data<AppState>, req: web::Json<UserData>) -> actix_web::Result<HttpResponse> {
    let user_service = &state.user_service;
    let user = req.into_inner().into();
    let result = user_service.create(user)?/*.await?*/;
    Ok(HttpResponse::Created().json(UserResponse::from(result)))
}

#[get("/users")]
async fn get_users(state: web::Data<AppState>) -> actix_web::Result<HttpResponse> {
    let user_service = &state.user_service;
    let users = user_service.list()?/*.await?*/;
    let response: Vec<UserResponse> = users.into_iter().map(UserResponse::from).collect();

    Ok(HttpResponse::Ok().json(response))
}
