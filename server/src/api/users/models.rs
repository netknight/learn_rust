use std::backtrace::Backtrace;
use apistos::ApiComponent;
use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use commons::errors::ServiceError;
use commons::refined;
use crate::users::domain::{User, UserEntity, UserId};

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent)]
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
            name: refined::Username::try_new(self.name)
                .map_err(|e|ServiceError::from_validation_error(e, Backtrace::capture()))?,
            email: refined::Email::try_new(self.email)
                .map_err(|e|ServiceError::from_validation_error(e, Backtrace::capture()))?,
        })
    }
}


#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent)]
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

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct UsersResponse {
    users: Vec<UserResponse>
}

impl UsersResponse {
    pub fn new(users: Vec<UserResponse>) -> Self {
        Self { users }
    }
}

impl From<Vec<UserEntity>> for UsersResponse {
    fn from(entities: Vec<UserEntity>) -> Self {
        Self {
            users: entities.into_iter().map(UserResponse::from).collect()
        }
    }
}