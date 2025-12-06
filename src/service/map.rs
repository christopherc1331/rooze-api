use std::collections::HashSet;

use crate::{
    api::map::MapInfo,
    error::AppError,
    repository::{
        map::{MapRepository, types::MapStats},
        shop::ShopRepository,
    },
};

pub struct MapService {
    map_repo: MapRepository,
    shop_repo: ShopRepository,
}

impl MapService {
    pub fn new(map_repo: MapRepository, shop_repo: ShopRepository) -> Self {
        Self {
            map_repo,
            shop_repo,
        }
    }

    pub async fn get_map_info(
        &self,
        boundary: crate::repository::map::types::GeoBoundary,
        style_ids: Option<Vec<i64>>,
    ) -> Result<MapInfo, AppError> {
        let shops = self
            .shop_repo
            .get_shops_in_bounds(boundary, style_ids)
            .await?;

        let shop_count = shops.len() as i64;
        let artist_count = shops.iter().map(|s| s.artists.len() as i64).sum();
        let style_count = shops
            .iter()
            .flat_map(|s| s.artists.iter())
            .flat_map(|a| a.styles.iter())
            .map(|s| s.id)
            .collect::<HashSet<_>>()
            .len() as i64;

        let stats = MapStats {
            shop_count,
            artist_count,
            style_count,
        };

        Ok(MapInfo { stats, shops })
    }

    pub async fn get_cities(
        &self,
        state: String,
    ) -> Result<Vec<crate::repository::map::types::City>, AppError> {
        Ok(self.map_repo.get_cities(state).await?)
    }

    pub async fn get_states(&self) -> Result<Vec<crate::repository::map::types::State>, AppError> {
        Ok(self.map_repo.get_states().await?)
    }

    pub async fn get_locations_with_details(
        &self,
        boundary: crate::repository::map::types::GeoBoundary,
        style_ids: Option<Vec<i64>>,
        states: Option<Vec<String>>,
        cities: Option<Vec<String>>,
    ) -> Result<Vec<crate::repository::map::types::LocationWithDetails>, AppError> {
        Ok(self
            .map_repo
            .get_locations_with_details(boundary, style_ids, states, cities)
            .await?)
    }

    pub async fn search_by_postal_code(
        &self,
        postal_code: String,
    ) -> Result<Option<crate::repository::map::types::PostalCodeResult>, AppError> {
        Ok(self.map_repo.search_by_postal_code(postal_code).await?)
    }

    pub async fn get_bounding_box(
        &self,
        city: Option<String>,
        state: String,
    ) -> Result<Option<crate::repository::map::types::BoundingBox>, AppError> {
        Ok(self
            .map_repo
            .get_bounding_box_for_location(city, state)
            .await?)
    }

    pub async fn get_bounding_box_by_postal_code(
        &self,
        postal_code: String,
    ) -> Result<Option<crate::repository::map::types::BoundingBox>, AppError> {
        Ok(self
            .map_repo
            .get_bounding_box_for_postal_code(postal_code)
            .await?)
    }
}
