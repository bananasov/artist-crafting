pub mod collector;
pub mod database;
pub mod errors;
pub mod middlewares;
pub mod routes;

use std::{env, sync::LazyLock};

use sea_orm::DatabaseConnection;
use totp_rs::{Algorithm, TOTP};

pub static TOTP_FUCKERY: LazyLock<TOTP> = LazyLock::new(|| {
    let totp_secret = env::var("TOTP_SECRET").expect("TOTP_SECRET is not set .env file");
    TOTP::new(
        Algorithm::SHA1,
        6,
        1,
        30,
        totp_secret.into_bytes(),
        None,
        "artist-crafting-nya".to_owned(),
    )
    .expect("Failed to create TOTP")
});

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}
