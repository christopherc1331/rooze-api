use poem_openapi::{OpenApi, payload::PlainText};

pub struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/health", method = "get")]
    async fn health(&self) -> PlainText<&'static str> {
        PlainText("Ok")
    }
}
