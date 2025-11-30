use std::sync::Arc;

use crate::error::AppError;
use crate::repository::styles::styles_repository::StylesRepository;
use crate::repository::styles::styles_types::StyleWithCount;

pub struct StylesService {
    repo: Arc<StylesRepository>,
}

impl StylesService {
    pub fn new(repo: Arc<StylesRepository>) -> Self {
        Self { repo }
    }

    pub async fn get_popular_styles(&self, limit: i64) -> Result<Vec<StyleWithCount>, AppError> {
        Ok(self.repo.get_popular_styles(limit).await?)
    }
}
