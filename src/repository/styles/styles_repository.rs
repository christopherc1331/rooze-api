use sea_orm::{DatabaseBackend::Postgres, DatabaseConnection, DbErr, FromQueryResult, Statement};

use crate::repository::styles::styles_types::{StyleTypeWithCount, StyleWithCount};

pub struct StylesRepository {
    db: DatabaseConnection,
}

impl StylesRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn get_popular_styles(&self, limit: i64) -> Result<Vec<StyleWithCount>, DbErr> {
        StyleWithCount::find_by_statement(Statement::from_sql_and_values(
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

    pub async fn get_styles_with_bounds(&self) -> Resullt<Vec<StyleTypeWithCount>> {
        StyleTypeWithCount::find_by_statement(Statement::from_sql_and_values(
            Postgres,
            r#"
                ;WITH artists_in_bounds AS (
                    SELECT a.id
                    FROM artists a
                    INNER JOIN locations l ON a.location_id = l.id
                    WHERE l.lat BETWEEN $1 AND $2
                    AND l.long BETWEEN $3 AND $4
                )
                SELECT
                    s.id,
                    s.name,
                    s.type as style_type,
                    COUNT(s.id) as style_count
                FROM artists_images_styles ais
                INNER JOIN artists_images ai ON ai.id = ais.artists_images_id
                INNER JOIN artists_in_bounds aib ON aib.id = ai.artist_id
                INNER JOIN styles s ON ais.style_id = s.id
                GROUP BY s.id, s.name, s.type
                ORDER BY s.type, s.name
            "#,
            [],
        ))
    }
}
