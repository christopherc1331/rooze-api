#![deny(warnings)]

use std::fs;

use poem::{Route, Server, listener::TcpListener};
use poem_openapi::OpenApiService;

mod api;

#[tokio::main]
async fn main() {
    let api_service = OpenApiService::new(api::Api, "Rooze API", "1.0");
    let yaml = api_service.spec_yaml();
    fs::write("openapi.yaml", &yaml).expect("Unable to write OpenAPI spec to file");

    let app = Route::new().nest("/api", api_service);

    let port = dotenv::var("PORT").unwrap_or("9090".to_string());
    let addr = format!("0.0.0.0:{}", port);
    println!("Listening on http://{}", addr);

    Server::new(TcpListener::bind(&addr))
        .run(app)
        .await
        .expect("Server to run");
}
