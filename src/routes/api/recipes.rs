use actix_web::{HttpResponse, get, web};

#[get("")]
async fn recipes_index() -> HttpResponse {
    HttpResponse::Ok().body("hi")
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/recipes").service(recipes_index));
}
