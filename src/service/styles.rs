use crate::{error::AppError, repository::styles::StylesRepository};

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
    ) -> Result<Vec<crate::repository::styles::types::StyleWithCount>, AppError> {
        Ok(self.repo.get_popular_styles(limit).await?)
    }

    pub async fn get_styles_with_bounds(
        &self,
        boundary: crate::repository::map::types::GeoBoundary,
    ) -> Result<Vec<crate::repository::styles::types::StyleTypeWithCount>, AppError> {
        Ok(self.repo.get_styles_with_bounds(boundary).await?)
    }

    pub async fn get_all_styles_with_counts(
        &self,
        selected_style_ids: Option<Vec<i64>>,
    ) -> Result<Vec<crate::repository::styles::types::StyleTypeWithCount>, AppError> {
        Ok(self
            .repo
            .get_all_styles_with_counts(selected_style_ids)
            .await?)
    }

    pub async fn get_filtered_styles_with_counts(
        &self,
        selected_style_ids: Option<Vec<i64>>,
    ) -> Result<Vec<crate::repository::styles::types::StyleTypeWithCount>, AppError> {
        Ok(self
            .repo
            .get_filtered_styles_with_counts(selected_style_ids)
            .await?)
    }
}
