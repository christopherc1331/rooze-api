use poem_openapi::{OpenApi, param::Query, payload::Json};

use crate::{
    error::ErrorResponse,
    repository::map::{MapRepository, types::MapState},
    service::MapService,
};

pub struct MapApi {
    service: MapService,
}

#[OpenApi(prefix_path = "/map")]
impl MapApi {
    pub fn new(state: std::sync::Arc<crate::AppState>) -> Self {
        let repo = MapRepository::new(state.db.clone());
        let service = MapService::new(repo);
        Self { service }
    }

    #[oai(path = "/popular", method = "get")]
    async fn get_map_state_for_bounds(
        &self,
        south_west_lat: Query<f64>,
        north_east_lat: Query<f64>,
        south_west_long: Query<f64>,
        north_east_long: Query<f64>,
    ) -> Result<Json<Option<MapState>>, ErrorResponse> {
        let _boundary = crate::repository::map::types::GeoBoundary {
            south_west_lat: south_west_lat.0,
            north_east_lat: north_east_lat.0,
            south_west_long: south_west_long.0,
            north_east_long: north_east_long.0,
        };
        let map_state = self
            .service
            .get_map_state_in_bounds(_boundary)
            .await
            .map_err(ErrorResponse::from)?;

        Ok(Json(map_state))
    }
}
