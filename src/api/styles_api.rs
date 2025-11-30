use crate::{
    error::ErrorResponse,
    repository::styles::{styles_repository::StylesRepository, styles_types::StyleWithCount},
    service::styles_service::StylesService,
};
use poem_openapi::{OpenApi, param::Query, payload::Json};

pub struct StylesApi {
    service: StylesService,
}

#[OpenApi(prefix_path = "/styles")]
impl StylesApi {
    pub fn new(state: std::sync::Arc<crate::AppState>) -> Self {
        let repo = StylesRepository::new(state.db.clone());
        let service = StylesService::new(repo);
        Self { service }
    }

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

    #[oai(path = "/by_bounds", method = "get")]
    async fn get_styles_by_bounds(
        &self,
        south_west_lat: Query<f64>,
        north_east_lat: Query<f64>,
        south_west_long: Query<f64>,
        north_east_long: Query<f64>,
    ) -> Result<Json<Vec<crate::repository::styles::styles_types::StyleTypeWithCount>>, ErrorResponse>
    {
        let boundary = crate::repository::styles::styles_types::GeoBoundary {
            south_west_lat: south_west_lat.0,
            north_east_lat: north_east_lat.0,
            south_west_long: south_west_long.0,
            north_east_long: north_east_long.0,
        };
        let styles = self
            .service
            .get_styles_with_bounds(boundary)
            .await
            .map_err(ErrorResponse::from)?;
        Ok(Json(styles))
    }
}
