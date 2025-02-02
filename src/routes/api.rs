pub mod items;
pub mod recipes;
pub mod tags;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.configure(items::config);
    cfg.configure(recipes::config);
    cfg.configure(tags::config);
}
