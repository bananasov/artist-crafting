use actix_web::{HttpResponse, get, web};

#[get("/")]
async fn tags_index() -> HttpResponse {
    HttpResponse::Ok().body("hi")
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/tags").service(tags_index));
}
