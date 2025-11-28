use crate::{
    api::builder::ApiModule, repository::styles_repository::StylesRepository,
    service::styles_service::StylesService,
};
use poem_openapi::{Object, OpenApi, param::Query, payload::Json};

#[derive(Object)]
pub struct Style {
    pub name: String,
}

pub struct StylesApi {
    service: StylesService,
}

#[OpenApi(prefix_path = "/styles")]
impl StylesApi {
    #[oai(path = "/popular", method = "get")]
    async fn get_popular_styles(&self, limit: Query<usize>) -> Json<Vec<Style>> {
        let styles = self
            .service
            .get_popular_styles(limit.0)
            .into_iter()
            .map(|name| Style { name })
            .collect();
        Json(styles)
    }
}

impl ApiModule for StylesApi {
    type Api = StylesApi;

    fn build(state: std::sync::Arc<crate::AppState>) -> Self::Api {
        let repo = std::sync::Arc::new(StylesRepository::new(state.db.clone()));
        let service = StylesService::new(repo);
        StylesApi { service }
    }
}
