use actix_settings::BasicSettings;
use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub struct AppSettings {
    pub name: String,
}

pub type Settings = BasicSettings<AppSettings>;

pub fn load<P: AsRef<Path>>(path: P) -> Settings {
    let path_str = path.as_ref().display().to_string();
    Settings::parse_toml(path).unwrap_or_else(|_| panic!("Failed to load settings from {path_str}"))
}
