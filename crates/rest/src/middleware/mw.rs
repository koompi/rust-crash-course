use super::auth;
use actix_web::{
    dev::{self, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures_util::future::LocalBoxFuture;
use std::future::{ready, Ready};

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct Authorization;

// Middleware factory is `Transform` trait from actix-service crate
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for Authorization
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = SayHiMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(SayHiMiddleware { service }))
    }
}

pub struct SayHiMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for SayHiMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        println!("Hi from start. You requested: {}", req.path());

        let authorization = req.headers().get("Authorization");
        if authorization.is_none() {
            return Box::pin(async {
                Err(actix_web::error::ErrorUnauthorized(
                    "Unauthorized. Login required",
                ))
            });
        }

        let bearer_token = authorization.unwrap().to_str().unwrap().to_owned();
        if !bearer_token.contains("Bearer ") {
            return Box::pin(async {
                Err(actix_web::error::ErrorUnauthorized(
                    "Unauthorized. Token must be of Bearer type",
                ))
            });
        }

        let token = bearer_token.replace("Bearer ", "");

        let decoded = auth::verify_token(token);
        if decoded.is_none() {
            return Box::pin(async {
                Err(actix_web::error::ErrorUnauthorized(
                    "Unauthorized. Invalid JWT token",
                ))
            });
        }

        Box::pin(self.service.call(req))
    }
}
