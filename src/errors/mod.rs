pub mod totp;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorResponse<'a> {
    pub status: u16,
    pub message: String,
    pub code: &'a str,
    pub timestamp: String,
}

pub trait ErrorExt {
    fn error_code(&self) -> &str;
}
