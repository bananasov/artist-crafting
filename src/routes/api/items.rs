use actix_web::{HttpResponse, get, web};

#[get("")]
async fn items_index() -> HttpResponse {
    HttpResponse::Ok().body("hi")
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/items").service(items_index));
}
