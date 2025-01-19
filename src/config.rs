use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Settings {
    pub server: ServerSettings,
    pub proxy: ProxySettings,
    pub auth: AuthSettings,
    pub logging: LoggingSettings,
}

#[derive(Deserialize, Debug)]
pub struct ServerSettings {
    pub address: String,
    pub port: u16,
}

#[derive(Deserialize, Debug)]
pub struct ProxySettings {
    pub target_url: String,
    pub proxy_ips: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct AuthSettings {
    pub enable_auth: bool,
    pub tokens: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct LoggingSettings {
    pub level: String,
    pub file: String,
}

impl Settings {
    pub fn new() -> Result<Self, config::ConfigError> {
        let mut cfg = config::Config::default();
        cfg.merge(config::File::with_name("config/config"))?;
        cfg.try_into()
    }
}
