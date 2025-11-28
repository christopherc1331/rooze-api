#![deny(warnings)]

use std::fs;

use poem::{EndpointExt, Route, Server, listener::TcpListener};
use poem_openapi::OpenApiService;
use sea_orm::{Database, DatabaseConnection};

mod api;
mod entity;
mod repository;
mod service;
mod util;

#[tokio::main]
async fn main() {
    let api_service = OpenApiService::new((api::HealthApi, api::StylesApi), "Rooze API", "1.0");
    let yaml = api_service.spec_yaml();
    fs::write("openapi.yaml", &yaml).expect("Unable to write OpenAPI spec to file");

    let db_path = util::get_env_var("DATABASE_URL", "");
    let db: DatabaseConnection = Database::connect(db_path)
        .await
        .expect("Failed to connect to the database");

    let app = Route::new().nest("/api", api_service).data(db);

    let port = util::get_env_var("PORT", "8080");
    let addr = format!("0.0.0.0:{}", port);
    println!("Listening on http://{}", addr);

    Server::new(TcpListener::bind(&addr))
        .run(app)
        .await
        .expect("Server to run");
}
