use crate::{
    api::builder::ApiModule,
    error::ErrorResponse,
    repository::styles::{styles_repository::StylesRepository, styles_types::StyleWithCount},
    service::styles_service::StylesService,
};
use poem_openapi::{OpenApi, param::Query, payload::Json};

pub struct StylesApi {
    service: StylesService,
}

impl ApiModule for StylesApi {
    type Api = StylesApi;

    fn build(state: std::sync::Arc<crate::AppState>) -> Self::Api {
        let repo = std::sync::Arc::new(StylesRepository::new(state.db.clone()));
        let service = StylesService::new(repo);
        StylesApi { service }
    }
}

#[OpenApi(prefix_path = "/styles")]
impl StylesApi {
    #[oai(path = "/popular", method = "get")]
    async fn get_popular_styles(
        &self,
        limit: Query<i64>,
    ) -> Result<Json<Vec<StyleWithCount>>, ErrorResponse> {
        let styles = self
            .service
            .get_popular_styles(limit.0)
            .await
            .map_err(ErrorResponse::from)?;
        Ok(Json(styles))
    }
}
