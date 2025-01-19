use hyper::{Body, Request, Response};
use tower::Service;
use std::task::{Context, Poll};
use std::future::Future;
use std::pin::Pin;
use log::info;

pub struct LoggingMiddleware<S> {
    inner: S,
}

impl<S> LoggingMiddleware<S> {
    pub fn new(inner: S) -> Self {
        Self { inner }
    }
}

impl<S, ReqBody, ResBody> Service<Request<ReqBody>> for LoggingMiddleware<S>
where
    S: Service<Request<ReqBody>, Response = Response<ResBody>> + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<ReqBody>) -> Self::Future {
        info!("Received request: {} {}", req.method(), req.uri());
        let fut = self.inner.call(req);
        Box::pin(async move {
            let res = fut.await;
            res
        })
    }
}
