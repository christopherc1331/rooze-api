#![deny(warnings)]

use std::{fs, sync::Arc};

use poem::{Route, Server, listener::TcpListener};
use poem_openapi::OpenApiService;
use sea_orm::{Database, DatabaseConnection};

use crate::api::StylesApi;

mod api;
mod entity;
mod error;
mod repository;
mod service;
mod util;

pub struct AppState {
    pub db: DatabaseConnection,
}

fn build_app(state: Arc<AppState>) -> Route {
    let styles_api = StylesApi::new(state.clone());
    let api_service = OpenApiService::new((api::HealthApi, styles_api), "Rooze API", "1.0");
    let yaml = api_service.spec_yaml();
    fs::write("openapi.yaml", &yaml).expect("Unable to write OpenAPI spec to file");

    Route::new().nest("/api", api_service)
}

#[tokio::main]
async fn main() {
    let db_path = util::get_env_var("DATABASE_URL", "");
    let db: DatabaseConnection = Database::connect(db_path)
        .await
        .expect("Failed to connect to the database");

    let app = build_app(Arc::new(AppState { db }));

    let port = util::get_env_var("PORT", "8080");
    let addr = format!("0.0.0.0:{}", port);
    println!("Listening on http://{}", addr);

    Server::new(TcpListener::bind(&addr))
        .run(app)
        .await
        .expect("Server to run");
}
