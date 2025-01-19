use dashmap::DashSet;
use lazy_static::lazy_static;
use tokio::sync::RwLock;
use std::sync::Arc;

lazy_static! {
    static ref VALID_TOKENS: DashSet<String> = DashSet::new();
}

pub async fn authenticate(token: &str, tokens: &Arc<Vec<String>>) -> bool {
    if VALID_TOKENS.is_empty() {
        for t in tokens.iter() {
            VALID_TOKENS.insert(t.clone());
        }
    }
    VALID_TOKENS.contains(token)
}
