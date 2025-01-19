use crate::config::Settings;
use crate::auth::authenticate;
use crate::utils::get_next_proxy_ip;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Client, Request, Response, Server, Uri};
use log::{info, error};
use std::convert::Infallible;
use std::net::SocketAddr;
use tokio::sync::RwLock;
use std::sync::Arc;

pub async fn start_proxy(settings: Settings) -> Result<(), Box<dyn std::error::Error>> {
    let addr = SocketAddr::new(
        settings.server.address.parse()?,
        settings.server.port,
    );

    let client = Client::new();
    let proxy_ips = Arc::new(RwLock::new(settings.proxy.proxy_ips));
    let target_url = settings.proxy.target_url.clone();
    let enable_auth = settings.auth.enable_auth;
    let valid_tokens = Arc::new(settings.auth.tokens);

    let make_svc = make_service_fn(move |_| {
        let client = client.clone();
        let proxy_ips = proxy_ips.clone();
        let target_url = target_url.clone();
        let enable_auth = enable_auth;
        let valid_tokens = valid_tokens.clone();
        async move {
            Ok::<_, Infallible>(service_fn(move |req: Request<Body>| {
                let client = client.clone();
                let proxy_ips = proxy_ips.clone();
                let target_url = target_url.clone();
                let enable_auth = enable_auth;
                let valid_tokens = valid_tokens.clone();
                async move {
                    if enable_auth {
                        if let Some(auth_header) = req.headers().get("Authorization") {
                            let token = auth_header.to_str().unwrap_or("");
                            if !authenticate(token, &valid_tokens).await {
                                return Ok::<_, Infallible>(
                                    Response::builder()
                                        .status(401)
                                        .body(Body::from("Unauthorized"))
                                        .unwrap(),
                                );
                            }
                        } else {
                            return Ok::<_, Infallible>(
                                Response::builder()
                                    .status(401)
                                    .body(Body::from("Unauthorized"))
                                    .unwrap(),
                            );
                        }
                    }

                    let proxy_ip = get_next_proxy_ip(&proxy_ips).await;
                    info!("Using proxy IP: {}", proxy_ip);
                

                    let uri = format!("{}{}", target_url, req.uri()).parse::<Uri>()?;
                    let mut proxied_req = req;
                    *proxied_req.uri_mut() = uri;
                    let response = client.request(proxied_req).await?;
                    Ok::<_, Infallible>(response)
                }
            }))
        }
    });

    info!("Starting proxy server at http://{}", addr);
    let server = Server::bind(&addr).serve(make_svc);
    server.await?;
    Ok(())
}
