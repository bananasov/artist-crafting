use std::env;

use actix_web::{App, HttpResponse, HttpServer, get, middleware, web};
use artist_crafting::{AppState, routes};
use sea_orm::Database;

#[get("/")]
async fn hello() -> Result<HttpResponse, actix_web::Error> {
    Ok(HttpResponse::Ok().body("Hello world!"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();

    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");

    let conn = Database::connect(&db_url)
        .await
        .expect("Failed to connect to the database");

    let state = AppState { db: conn };

    tracing::info!("Starting HTTP server on {server_url}");

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::NormalizePath::trim())
            .app_data(web::Data::new(state.clone()))
            .configure(routes::config)
            .service(hello)
    })
    .bind(server_url)?
    .run()
    .await
}
