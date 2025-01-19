use tokio::sync::RwLock;
use std::sync::Arc;
use rand::seq::SliceRandom;

pub async fn get_next_proxy_ip(proxy_ips: &Arc<RwLock<Vec<String>>>) -> String {
    let ips = proxy_ips.read().await;
    let mut rng = rand::thread_rng();
    ips.choose(&mut rng).unwrap_or(&"0.0.0.0".to_string()).clone()
}

pub fn generate_uuid() -> String {
    uuid::Uuid::new_v4().to_string()
}
