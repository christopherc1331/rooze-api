use poem_openapi::Object;
use sea_orm::{DatabaseBackend::Postgres, DatabaseConnection, DbErr, FromQueryResult, Statement};

#[derive(Debug, FromQueryResult, Object)]
pub struct PopularStyle {
    pub count: i64,
    pub id: i64,
    pub name: String,
}

pub struct StylesRepository {
    db: DatabaseConnection,
}

impl StylesRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn get_popular_styles(&self, limit: i64) -> Result<Vec<PopularStyle>, DbErr> {
        PopularStyle::find_by_statement(Statement::from_sql_and_values(
            Postgres,
            r#"
                    SELECT COUNT(s.name), s.id, s.name
                    FROM artists_images_styles ais
                    INNER JOIN styles s ON ais.style_id = s.id
                    GROUP BY s.name, s.id
                    ORDER BY COUNT(s.name) DESC
                    LIMIT $1
                "#,
            [limit.into()],
        ))
        .all(&self.db)
        .await
    }
}
