use std::sync::Arc;

use crate::error::AppError;
use crate::repository::styles_repository::{PopularStyle, StylesRepository};

pub struct StylesService {
    repo: Arc<StylesRepository>,
}

impl StylesService {
    pub fn new(repo: Arc<StylesRepository>) -> Self {
        Self { repo }
    }

    pub async fn get_popular_styles(&self, limit: i64) -> Result<Vec<PopularStyle>, AppError> {
        Ok(self.repo.get_popular_styles(limit).await?)
    }
}
