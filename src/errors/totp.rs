use super::{ErrorExt, ErrorResponse};
use actix_web::{HttpResponse, body::BoxBody, error, http::StatusCode};
use chrono::Utc;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TOTPError {
    #[error("Invalid or missing TOTP token")]
    Unauthorized,

    #[error(transparent)]
    Actix(#[from] actix_web::Error),
}

impl error::ResponseError for TOTPError {
    fn status_code(&self) -> StatusCode {
        StatusCode::UNAUTHORIZED
    }

    fn error_response(&self) -> actix_web::HttpResponse<BoxBody> {
        let status_code = self.status_code();
        let response = ErrorResponse {
            status: status_code.as_u16(),
            message: self.to_string(),
            code: self.error_code(),
            timestamp: Utc::now().to_rfc3339(),
        };

        HttpResponse::build(status_code).json(response)
    }
}

impl super::ErrorExt for TOTPError {
    fn error_code(&self) -> &str {
        match self {
            TOTPError::Unauthorized => "TOTP_UNAUTHORIZED",
            TOTPError::Actix(_) => "WEB_FRAMEWORK",
        }
    }
}
