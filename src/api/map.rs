use poem_openapi::{
    Object, OpenApi,
    param::{Path, Query},
    payload::Json,
};

use crate::{
    error::ErrorResponse,
    repository::{
        map::{
            MapRepository,
            types::{BoundingBox, City, LocationWithDetails, MapStats, PostalCodeResult, State},
        },
        shop::{ShopRepository, types::ShopWithDetails},
    },
    service::MapService,
};

pub struct MapApi {
    service: MapService,
}

#[derive(Debug, Object)]
pub struct MapInfo {
    pub stats: MapStats,
    pub shops: Vec<ShopWithDetails>,
}

#[OpenApi(prefix_path = "/map")]
impl MapApi {
    pub fn new(state: std::sync::Arc<crate::AppState>) -> Self {
        let map_repo = MapRepository::new(state.db.clone());
        let shop_repo = ShopRepository::new(state.db.clone());
        let service = MapService::new(map_repo, shop_repo);
        Self { service }
    }

    #[oai(path = "/", method = "get")]
    async fn get_map_info_for_bounds(
        &self,
        south_west_lat: Query<f64>,
        north_east_lat: Query<f64>,
        south_west_long: Query<f64>,
        north_east_long: Query<f64>,
        style_ids: Query<Option<String>>,
    ) -> Result<Json<MapInfo>, ErrorResponse> {
        let boundary = crate::repository::map::types::GeoBoundary {
            south_west_lat: south_west_lat.0,
            north_east_lat: north_east_lat.0,
            south_west_long: south_west_long.0,
            north_east_long: north_east_long.0,
        };
        let style_ids_parsed = style_ids.0.map(|s| {
            s.split(',')
                .filter_map(|id| id.trim().parse::<i64>().ok())
                .collect()
        });
        let map_info = self
            .service
            .get_map_info(boundary, style_ids_parsed)
            .await
            .map_err(ErrorResponse::from)?;

        Ok(Json(map_info))
    }

    #[oai(path = "/cities", method = "get")]
    async fn get_cities(&self, state: Query<String>) -> Result<Json<Vec<City>>, ErrorResponse> {
        let cities = self
            .service
            .get_cities(state.0)
            .await
            .map_err(ErrorResponse::from)?;
        Ok(Json(cities))
    }

    #[oai(path = "/states", method = "get")]
    async fn get_states(&self) -> Result<Json<Vec<State>>, ErrorResponse> {
        let states = self
            .service
            .get_states()
            .await
            .map_err(ErrorResponse::from)?;
        Ok(Json(states))
    }

    #[oai(path = "/locations", method = "get")]
    #[allow(clippy::too_many_arguments)]
    async fn get_locations(
        &self,
        south_west_lat: Query<f64>,
        north_east_lat: Query<f64>,
        south_west_long: Query<f64>,
        north_east_long: Query<f64>,
        style_ids: Query<Option<String>>,
        states: Query<Option<String>>,
        cities: Query<Option<String>>,
    ) -> Result<Json<Vec<LocationWithDetails>>, ErrorResponse> {
        let boundary = crate::repository::map::types::GeoBoundary {
            south_west_lat: south_west_lat.0,
            north_east_lat: north_east_lat.0,
            south_west_long: south_west_long.0,
            north_east_long: north_east_long.0,
        };
        let style_ids_parsed = style_ids.0.map(|s| {
            s.split(',')
                .filter_map(|id| id.trim().parse::<i64>().ok())
                .collect()
        });
        let states_parsed = states.0.map(|s| {
            s.split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect()
        });
        let cities_parsed = cities.0.map(|s| {
            s.split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect()
        });
        let locations = self
            .service
            .get_locations_with_details(boundary, style_ids_parsed, states_parsed, cities_parsed)
            .await
            .map_err(ErrorResponse::from)?;
        Ok(Json(locations))
    }

    #[oai(path = "/postal_code/:code", method = "get")]
    async fn search_postal_code(
        &self,
        code: Path<String>,
    ) -> Result<Json<Option<PostalCodeResult>>, ErrorResponse> {
        let result = self
            .service
            .search_by_postal_code(code.0)
            .await
            .map_err(ErrorResponse::from)?;
        Ok(Json(result))
    }

    #[oai(path = "/bounds", method = "get")]
    async fn get_bounding_box(
        &self,
        state: Query<String>,
        city: Query<Option<String>>,
    ) -> Result<Json<Option<BoundingBox>>, ErrorResponse> {
        let result = self
            .service
            .get_bounding_box(city.0, state.0)
            .await
            .map_err(ErrorResponse::from)?;
        Ok(Json(result))
    }

    #[oai(path = "/bounds/postal_code/:code", method = "get")]
    async fn get_bounding_box_by_postal_code(
        &self,
        code: Path<String>,
    ) -> Result<Json<Option<BoundingBox>>, ErrorResponse> {
        let result = self
            .service
            .get_bounding_box_by_postal_code(code.0)
            .await
            .map_err(ErrorResponse::from)?;
        Ok(Json(result))
    }
}
