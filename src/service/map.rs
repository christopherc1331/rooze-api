use crate::{error::AppError, repository::map::MapRepository};

pub struct MapService {
    repo: MapRepository,
}

impl MapService {
    pub fn new(repo: MapRepository) -> Self {
        Self { repo }
    }

    pub async fn get_map_state_in_bounds(
        &self,
        boundary: crate::repository::map::types::GeoBoundary,
    ) -> Result<Option<crate::repository::map::types::MapState>, AppError> {
        // TODO: fetching map stats, also need to fetch locations from location repository,
        // then update controller/service method sig to match map state + location vector
        Ok(self.repo.get_map_state_for_bounds(boundary).await?)
    }

    pub async fn get_cities(
        &self,
        state: String,
    ) -> Result<Vec<crate::repository::map::types::City>, AppError> {
        Ok(self.repo.get_cities(state).await?)
    }

    pub async fn get_states(&self) -> Result<Vec<crate::repository::map::types::State>, AppError> {
        Ok(self.repo.get_states().await?)
    }

    pub async fn get_locations_with_details(
        &self,
        boundary: crate::repository::map::types::GeoBoundary,
        style_ids: Option<Vec<i64>>,
        states: Option<Vec<String>>,
        cities: Option<Vec<String>>,
    ) -> Result<Vec<crate::repository::map::types::LocationWithDetails>, AppError> {
        Ok(self
            .repo
            .get_locations_with_details(boundary, style_ids, states, cities)
            .await?)
    }

    pub async fn search_by_postal_code(
        &self,
        postal_code: String,
    ) -> Result<Option<crate::repository::map::types::PostalCodeResult>, AppError> {
        Ok(self.repo.search_by_postal_code(postal_code).await?)
    }
}
