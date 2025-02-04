pub mod api;
pub mod collector;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api").configure(api::config));
    cfg.service(web::scope("/collector").configure(collector::config));
}
