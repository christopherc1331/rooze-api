use poem_openapi::{OpenApi, param::Query, payload::Json};

use crate::{
    error::ErrorResponse,
    repository::styles::{StylesRepository, types::StyleWithCount},
    service::StylesService,
};

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
    ) -> Result<Json<Vec<crate::repository::styles::types::StyleTypeWithCount>>, ErrorResponse>
    {
        let boundary = crate::repository::map::types::GeoBoundary {
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

    /// Get all styles with artist counts, grouped by type
    #[oai(path = "/all", method = "get")]
    async fn get_all_styles(
        &self,
        style_ids: Query<Option<String>>,
    ) -> Result<Json<Vec<crate::repository::styles::types::StyleTypeWithCount>>, ErrorResponse>
    {
        let style_ids_parsed = style_ids.0.map(|s| {
            s.split(',')
                .filter_map(|id| id.trim().parse::<i64>().ok())
                .collect()
        });
        let styles = self
            .service
            .get_all_styles_with_counts(style_ids_parsed)
            .await
            .map_err(ErrorResponse::from)?;
        Ok(Json(styles))
    }

    /// Get styles that co-occur with selected styles (for filtering)
    #[oai(path = "/filtered", method = "get")]
    async fn get_filtered_styles(
        &self,
        style_ids: Query<Option<String>>,
    ) -> Result<Json<Vec<crate::repository::styles::types::StyleTypeWithCount>>, ErrorResponse>
    {
        let style_ids_parsed = style_ids.0.map(|s| {
            s.split(',')
                .filter_map(|id| id.trim().parse::<i64>().ok())
                .collect()
        });
        let styles = self
            .service
            .get_filtered_styles_with_counts(style_ids_parsed)
            .await
            .map_err(ErrorResponse::from)?;
        Ok(Json(styles))
    }
}
