use actix_web::http::{HeaderName, HeaderValue};
use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use future::{ok, LocalBoxFuture, Ready};
use futures::prelude::*;
use std::{
    cell::RefCell,
    rc::Rc,
    task::{Context, Poll},
};
use tracing::{span, trace, Instrument, Level};
use uuid::Uuid;

pub const REQUEST_ID_HEADER: &str = "x-request-id";

pub struct Tracer;

impl<S, B> Transform<S> for Tracer
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = TracerMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(TracerMiddleware {
            service: Rc::new(RefCell::new(service)),
        })
    }
}

pub struct TracerMiddleware<S> {
    pub service: Rc<RefCell<S>>,
}

impl<S, B> Service for TracerMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = S::Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let request_id = Uuid::new_v4().to_string();
        let root_span = span!(Level::INFO, "HttpRequest", %request_id, method = req.method().as_str(), path = req.path(), query = req.query_string(), remote_ip = req.connection_info().remote_addr().unwrap_or("-"));

        let fut = self.service.call(req);
        Box::pin(
            async move {
                let mut res = fut.await?;

                res.headers_mut().insert(
                    HeaderName::from_static(REQUEST_ID_HEADER),
                    HeaderValue::from_str(&request_id).unwrap(),
                );
                trace!("request");

                Ok(res)
            }
            .instrument(root_span),
        )
    }
}
