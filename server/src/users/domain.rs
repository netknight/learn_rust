use chrono::{DateTime, Utc};
use uuid::Uuid;
use commons::entity::{Entity, EntityWithTimestamp};
use commons::errors::ValidationError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct User {
    pub name: String,
    pub email: String
}

impl User {
    pub fn new(name: String, email: String) -> Result<Self, ValidationError> {
        if name.is_empty() {
            return Err(ValidationError {
                field: "name".to_string(),
                message: "Name cannot be empty".to_string()
            });
        }

        if email.is_empty() {
            return Err(ValidationError {
                field: "email".to_string(),
                message: "Email cannot be empty".to_string()
            });
        }

        Ok(Self {
            name,
            email
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize, schemars::JsonSchema, apistos::ApiComponent)]
pub struct UserId(Uuid);

impl UserId {
    fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserEntity {
    pub id: UserId,
    pub name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl UserEntity {
    pub fn new(user: User) -> Self {
        Self {
            id: UserId::new(),
            name: user.name,
            email: user.email,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

impl Entity<UserId> for UserEntity {
    fn id(&self) -> &UserId {
        &self.id
    }
}

impl EntityWithTimestamp<UserId> for UserEntity {
    fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }
}

impl Into<User> for UserEntity {
    fn into(self) -> User {
        User {
            name: self.name,
            email: self.email
        }
    }
}
