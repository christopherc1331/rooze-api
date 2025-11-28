use sea_orm::DatabaseConnection;

pub struct StylesRepository {
    db: DatabaseConnection,
}

impl StylesRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub fn get_popular_styles(&self, limit: Option<usize>) -> Vec<String> {
        todo!("Implement database query to fetch popular styles")
    }
}
