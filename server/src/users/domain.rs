use chrono::{DateTime, Utc};
use uuid::Uuid;
use commons::{refined, entity};

#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub name: refined::Username,
    pub email: refined::Email
}

#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct UserId(Uuid);

impl UserId {
    fn new() -> Self {
        Self(Uuid::new_v4())
    }
}


pub struct UserEntity {
    pub id: UserId,
    pub name: refined::Username,
    pub email: refined::Email,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>
}

/*
derive_with_id!(UserEntity, UserId);
derive_with_created!(UserEntity);
derive_with_updated!(UserEntity);
derive_with_deleted!(UserEntity);
*/

impl UserEntity {
    pub fn new(user: User) -> Self {
        Self {
            id: UserId::new(),
            name: user.name,
            email: user.email,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None
        }
    }
}

impl entity::WithId<UserId> for UserEntity {
    fn id(&self) -> &UserId {
        &self.id
    }
}

impl entity::WithCreated for UserEntity {
    fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }
}

impl entity::WithUpdated for UserEntity {
    fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }
}

impl entity::WithDeleted for UserEntity {
    fn deleted_at(&self) -> Option<&DateTime<Utc>> {
        self.deleted_at.as_ref()
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

