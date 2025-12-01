use crate::{error::AppError, repository::posts::PostsRepository};

pub struct PostsService {
    repo: PostsRepository,
}

impl PostsService {
    pub fn new(repo: PostsRepository) -> Self {
        Self { repo }
    }

    pub async fn get_posts_count(
        &self,
        styles: Vec<String>,
        states: Option<Vec<String>>,
        cities: Option<Vec<String>>,
    ) -> Result<crate::repository::posts::types::PostsCount, AppError> {
        Ok(self.repo.get_posts_count(styles, states, cities).await?)
    }

    pub async fn get_posts_by_style(
        &self,
        styles: Vec<String>,
        states: Option<Vec<String>>,
        cities: Option<Vec<String>>,
        limit: i64,
        offset: i64,
        user_id: Option<i64>,
    ) -> Result<crate::repository::posts::types::PaginatedPosts, AppError> {
        Ok(self
            .repo
            .get_posts_by_style(styles, states, cities, limit, offset, user_id)
            .await?)
    }
}
