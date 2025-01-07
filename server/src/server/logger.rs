use crate::server::settings;

pub fn configure(settings:  &settings::Settings) {
    env_logger::init();
    let name = &settings.application.name;
    log::info!("Logger initialisation completed for '{name}'");
}
