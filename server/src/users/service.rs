use chrono::Utc;
use log::info;
use commons::service::{CrudService, ServiceResult};
use crate::users::domain::{User, UserEntity, UserId};

pub trait UserService: CrudService<UserId, User, UserEntity> {
    fn list(&self) -> ServiceResult<Vec<UserEntity>>;
}

pub struct UserServiceImpl;

impl UserServiceImpl {
    pub fn new() -> Self {
        info!("Creating UserServiceImpl instance");
        Self
    }
}

impl CrudService<UserId, User, UserEntity> for UserServiceImpl {
    fn create(&self, _data: User) -> ServiceResult<UserEntity> {
        gen_entity()
    }

    fn read(&self, id: UserId) -> ServiceResult<UserEntity> {
        gen_entity().map(|entity| UserEntity { id, ..entity })
    }

    fn update(&self, id: UserId, _data: User) -> ServiceResult<UserEntity> {
        gen_entity().map(|entity| UserEntity { id, updated_at: Utc::now(), ..entity })
    }

    fn delete(&self, _id: UserId) -> ServiceResult<()> {
        Ok(())
    }
}

impl UserService for UserServiceImpl {
    fn list(&self) -> ServiceResult<Vec<UserEntity>> {
        Ok(vec![gen_entity()?])
    }
}

fn gen_user () -> ServiceResult<User> {
    User::new(
        "John Doe".to_string(),
        "test@test.com".to_string()
    ).map_err(|e| e.into())
}

fn gen_entity () -> ServiceResult<UserEntity> {
    Ok(UserEntity::new(gen_user()?))
}
