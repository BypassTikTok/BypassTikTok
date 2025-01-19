use hyper::{Body, Request, Response};
use crate::auth::authenticate;
use crate::utils::generate_uuid;
use std::sync::Arc;
use dashmap::DashMap;

pub async fn handle_request(req: Request<Body>, session_store: Arc<DashMap<String, String>>) -> Result<Response<Body>, hyper::Error> {
    let session_id = generate_uuid();
    session_store.insert(session_id.clone(), "active".to_string());
    Ok(Response::new(Body::from(format!("Session ID: {}", session_id))))
}
