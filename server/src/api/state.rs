use std::sync::Mutex;
use crate::server::settings::Settings;
use crate::users::service::{UserService, UserServiceImpl};

pub struct AppState {
    pub requests: Mutex<i32>,
    pub settings: Settings,
    pub user_service: Box<dyn UserService + Send + Sync>,
}

impl AppState {
    pub fn new(settings: Settings) -> Self {
        let user_service = Box::new(UserServiceImpl::new());
        Self {
            settings,
            requests: Mutex::new(0),
            user_service,
        }
    }
}
