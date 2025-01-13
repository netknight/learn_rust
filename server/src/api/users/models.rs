use apistos::ApiComponent;
use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::users::domain::{User, UserEntity, UserId};

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent)]
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
            name: entity.name,
            email: entity.email,
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