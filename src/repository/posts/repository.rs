use sea_orm::{DatabaseConnection, DbErr};

use super::types::{PaginatedPosts, PostsCount};

pub struct PostsRepository {
    #[allow(dead_code)]
    db: DatabaseConnection,
}

impl PostsRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn get_posts_count(
        &self,
        _styles: Vec<String>,
        _states: Option<Vec<String>>,
        _cities: Option<Vec<String>>,
    ) -> Result<PostsCount, DbErr> {
        todo!()
    }

    pub async fn get_posts_by_style(
        &self,
        _styles: Vec<String>,
        _states: Option<Vec<String>>,
        _cities: Option<Vec<String>>,
        _limit: i64,
        _offset: i64,
        _user_id: Option<i64>,
    ) -> Result<PaginatedPosts, DbErr> {
        todo!()
    }
}
