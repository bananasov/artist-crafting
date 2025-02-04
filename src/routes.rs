pub mod api;
pub mod collector;

use actix_web::web;

use crate::middlewares::totp::TOTPGuard;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api").configure(api::config));
    cfg.service(
        web::scope("/collector")
            .wrap(TOTPGuard)
            .configure(collector::config),
    );
}
