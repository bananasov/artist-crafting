pub mod api;
pub mod collector;

use actix_web::web;

use crate::TOTP_FUCKERY;
use crate::middlewares::totp::TOTPMiddleware;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api").configure(api::config));
    cfg.service(
        web::scope("/collector")
            .wrap(TOTPMiddleware::new(TOTP_FUCKERY.clone())) // CURSED AS FUCK BUT I LOVE CURSED STUFF nya~
            .configure(collector::config),
    );
}
