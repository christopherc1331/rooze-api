use poem_openapi::{OpenApi, Object, param::Query, payload::Json};

use crate::service::styles_service;

#[derive(Object)]
pub struct Style {
    pub name: String,
}

pub struct StylesApi;

#[OpenApi(prefix_path = "/styles")]
impl StylesApi {
    #[oai(path = "/popular", method = "get")]
    async fn get_popular_styles(&self, limit: Query<Option<usize>>) -> Json<Vec<Style>> {
        let limit = limit.0.unwrap_or(10);
        let styles = styles_service::get_popular_styles(limit)
            .into_iter()
            .map(|name| Style { name })
            .collect();
        Json(styles)
    }
}
