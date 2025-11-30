use sea_orm::{DatabaseConnection, DbErr, FromQueryResult, Statement};

use crate::repository::map::types::{GeoBoundary, MapState};

pub struct MapRepository {
    db: DatabaseConnection,
}

impl MapRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn get_map_state_for_bounds(
        &self,
        boundary: GeoBoundary,
    ) -> Result<Option<MapState>, DbErr> {
        let GeoBoundary {
            south_west_lat,
            north_east_lat,
            south_west_long,
            north_east_long,
        } = boundary;

        MapState::find_by_statement(Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
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
                    COUNT(s.id) as count
                FROM artists_images_styles ais
                INNER JOIN artists_images ai ON ai.id = ais.artists_images_id
                INNER JOIN artists_in_bounds aib ON aib.id = ai.artist_id
                INNER JOIN styles s ON ais.style_id = s.id
                GROUP BY s.id, s.name, s.type
                ORDER BY s.type, s.name
            "#,
            [
                south_west_lat.into(),
                north_east_lat.into(),
                south_west_long.into(),
                north_east_long.into(),
            ],
        ))
        .one(&self.db)
        .await
    }
}
