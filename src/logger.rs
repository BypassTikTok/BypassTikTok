use env_logger::Builder;
use log::LevelFilter;
use std::env;
use crate::config::Settings;

pub fn init_logger() {
    let settings = Settings::new().expect("Failed to load configuration");
    Builder::from_default_env()
        .filter(None, match settings.logging.level.to_lowercase().as_str() {
            "debug" => LevelFilter::Debug,
            "info" => LevelFilter::Info,
            "warn" => LevelFilter::Warn,
            "error" => LevelFilter::Error,
            _ => LevelFilter::Info,
        })
        .init();
}
