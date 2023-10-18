use actix_web::{dev::ServiceRequest, dev::ServiceResponse, dev::Transform, dev::Service, Error, HttpResponse,ResponseError};
use futures::future::{self, ok, Ready, Either};
use std::task::{Context, Poll};
use actix_web::http::StatusCode;
use std::fmt;

#[derive(Debug)]
enum MyError {
    MissingToken,
    InvalidToken,
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyError::MissingToken => write!(f, "Token is required."),
            MyError::InvalidToken => write!(f, "Token is invalid."),
        }
    }
}

impl ResponseError for MyError {
    // status code for response
    fn status_code(&self) -> StatusCode {
        StatusCode::UNAUTHORIZED
    }

    // error response body
    fn error_response(&self) -> HttpResponse {
        HttpResponse::Unauthorized().json(self.to_string())
    }
}

pub struct MyMiddleware;

// register middleware factory
impl<S, B> Transform<S, ServiceRequest> for MyMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = MyMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(MyMiddlewareService { service })
    }
}

pub struct MyMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for MyMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Either<S::Future, Ready<Result<Self::Response, Self::Error>>>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        println!("Hi from start of middleware!");

        // get token from request headers
        let token = match req.headers().get("token") {
            None => return Either::Right(future::ready(Err(Error::from(MyError::MissingToken)))),
            Some(token) => token,
        };
        // check if token is valid
        if let Ok(token_str) = token.to_str() {
            if token_str == "3394" {
                println!("Token is valid.");
                return Either::Left(self.service.call(req));
            }
        }

        println!("Token is invalid.");
        Either::Right(future::ready(Err(Error::from(MyError::InvalidToken))))
    }
}
