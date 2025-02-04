use crate::{TOTP_FUCKERY, errors::totp::TOTPError};
use actix_web::{
    Error,
    dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready},
};
use futures::future::{LocalBoxFuture, Ready, ready};

pub struct TOTPGuard;

impl<S, B> Transform<S, ServiceRequest> for TOTPGuard
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = TOTPGuardMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(TOTPGuardMiddleware { service }))
    }
}

pub struct TOTPGuardMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for TOTPGuardMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        if !is_valid_totp(&req) {
            let error = TOTPError::Unauthorized.into();
            Box::pin(async move { Err(error) })
        } else {
            let fut = self.service.call(req);
            Box::pin(fut)
        }
    }
}

fn is_valid_totp(req: &ServiceRequest) -> bool {
    let totp_code = req
        .headers()
        .get("x-totp-code")
        .or_else(|| req.headers().get("X-TOTP-Code"))
        .and_then(|h| h.to_str().ok());
    let totp_code_nya = match totp_code {
        Some(code) => code,
        None => return false,
    };

    tracing::info!("Received TOTP code: {}", totp_code_nya);

    TOTP_FUCKERY.check_current(totp_code_nya).unwrap_or(false)
}
