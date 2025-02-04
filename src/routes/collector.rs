use actix_web::{HttpResponse, get, web};

#[get("/refresh")]
async fn collector_refresh_data() -> HttpResponse {
    HttpResponse::Ok().body("Nya gotta impl")
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(collector_refresh_data);
}
