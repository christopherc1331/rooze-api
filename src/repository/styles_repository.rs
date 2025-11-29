use std::vec;

use sea_orm::{DatabaseConnection, sea_query::Query};

use crate::entity::styles;

pub struct StylesRepository {
    db: DatabaseConnection,
}

impl StylesRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub fn get_popular_styles(&self, limit: Option<usize>) -> Vec<String> {
        let stmt = Query::select().columns(vec![]).from(styles::Entity);
    }
}
