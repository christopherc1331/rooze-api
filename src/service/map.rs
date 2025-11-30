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
}
