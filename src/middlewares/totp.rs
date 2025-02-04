use std::{
    future::{Ready, ready},
    rc::Rc,
};

use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready};
use futures_util::future::LocalBoxFuture;
use totp_rs::TOTP;

use crate::errors::totp::TOTPError;

pub struct TOTPMiddleware {
    totp: Rc<TOTP>,
}

impl TOTPMiddleware {
    pub fn new(totp: TOTP) -> Self {
        Self {
            totp: Rc::new(totp),
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for TOTPMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type InitError = ();
    type Transform = TOTPMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(TOTPMiddlewareService {
            service,
            totp: self.totp.clone(),
        }))
    }
}

pub struct TOTPMiddlewareService<S> {
    service: S,
    totp: Rc<TOTP>,
}

impl<S, B> Service<ServiceRequest> for TOTPMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let totp = self.totp.clone();
        let auth = req.headers().get("X-TOTP-Code");
        if let Some(code) = auth {
            if let Ok(token_str) = code.to_str() {
                let res = totp.check_current(token_str);
                if let Ok(accepted) = res {
                    if accepted {
                        let fut = self.service.call(req);
                        return Box::pin(async move {
                            let res = fut.await?;

                            Ok(res)
                        });
                    }
                }
            }

            return Box::pin(async move { Err(TOTPError::Unauthorized.into()) });
        }

        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;

            Ok(res)
        })
    }
}
