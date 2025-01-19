mod config;
mod proxy;
mod auth;
mod logger;
mod utils;

use config::Settings;
use proxy::start_proxy;
use logger::init_logger;
use tokio;

#[tokio::main]
async fn main() {
    init_logger();
    let settings = Settings::new().expect("Failed to load configuration");
    start_proxy(settings).await.expect("Proxy failed");
}
