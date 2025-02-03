pub mod database;
pub mod errors;
pub mod routes;

use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}
