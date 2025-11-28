use std::sync::Arc;

use crate::repository::styles_repository::StylesRepository;

pub struct StylesService {
    repo: Arc<StylesRepository>,
}

impl StylesService {
    pub fn new(repo: Arc<StylesRepository>) -> Self {
        Self { repo }
    }
    pub fn get_popular_styles(&self, limit: usize) -> Vec<String> {
        self.repo.get_popular_styles(Some(limit))
    }
}
