use crate::error::AppError;
use crate::repository::styles::styles_repository::StylesRepository;

pub struct StylesService {
    repo: StylesRepository,
}

impl StylesService {
    pub fn new(repo: StylesRepository) -> Self {
        Self { repo }
    }

    pub async fn get_popular_styles(
        &self,
        limit: i64,
    ) -> Result<Vec<crate::repository::styles::styles_types::StyleWithCount>, AppError> {
        Ok(self.repo.get_popular_styles(limit).await?)
    }

    pub async fn get_styles_with_bounds(
        &self,
        boundary: crate::repository::styles::styles_types::GeoBoundary,
    ) -> Result<Vec<crate::repository::styles::styles_types::StyleTypeWithCount>, AppError> {
        Ok(self.repo.get_styles_with_bounds(boundary).await?)
    }
}
