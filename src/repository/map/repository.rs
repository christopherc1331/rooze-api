use sea_orm::{DatabaseConnection, DbErr, FromQueryResult, Statement};

use crate::repository::map::types::{
    BoundingBox, City, GeoBoundary, LocationWithDetails, MapStats, PostalCodeResult, State,
};

pub struct MapRepository {
    #[allow(dead_code)]
    db: DatabaseConnection,
}

impl MapRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    #[allow(dead_code)]
    pub async fn get_map_state_for_bounds(
        &self,
        boundary: &GeoBoundary,
    ) -> Result<Option<MapStats>, DbErr> {
        let GeoBoundary {
            south_west_lat,
            north_east_lat,
            south_west_long,
            north_east_long,
        } = boundary;

        MapStats::find_by_statement(Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            r#"
                ;WITH artists_in_bounds AS (
                    SELECT
                        a.id artist_id,
                        a.location_id
                    FROM artists a
                    INNER JOIN locations l ON a.location_id = l.id
                    WHERE l.lat BETWEEN $1 AND $2
                    AND l.long BETWEEN $3 AND $4
                )
                SELECT
                    COUNT(DISTINCT aib.location_id) shop_count,
                    COUNT(DISTINCT aib.artist_id) artist_count,
                    COUNT(DISTINCT ais.style_id) style_count
                FROM artists_in_bounds aib
                LEFT JOIN artists_images ai ON ai.artist_id = aib.artist_id
                LEFT JOIN artists_images_styles ais ON ai.id = ais.artists_images_id
            "#,
            [
                (*south_west_lat).into(),
                (*north_east_lat).into(),
                (*south_west_long).into(),
                (*north_east_long).into(),
            ],
        ))
        .one(&self.db)
        .await
    }

    pub async fn get_cities(&self, _state: String) -> Result<Vec<City>, DbErr> {
        todo!()
    }

    pub async fn get_states(&self) -> Result<Vec<State>, DbErr> {
        todo!()
    }

    pub async fn get_locations_with_details(
        &self,
        _boundary: GeoBoundary,
        _style_ids: Option<Vec<i64>>,
        _states: Option<Vec<String>>,
        _cities: Option<Vec<String>>,
    ) -> Result<Vec<LocationWithDetails>, DbErr> {
        todo!()
    }

    pub async fn search_by_postal_code(
        &self,
        _postal_code: String,
    ) -> Result<Option<PostalCodeResult>, DbErr> {
        todo!()
    }

    pub async fn get_bounding_box_for_location(
        &self,
        city: Option<String>,
        state: String,
    ) -> Result<Option<BoundingBox>, DbErr> {
        let (query, params): (&str, Vec<sea_orm::Value>) = match city {
            Some(ref c) => (
                r#"
                    SELECT
                        MAX(l.lat) as north_east_lat,
                        MAX(l.long) as north_east_long,
                        MIN(l.lat) as south_west_lat,
                        MIN(l.long) as south_west_long
                    FROM locations l
                    INNER JOIN artists a ON a.location_id = l.id
                    WHERE l.state = $1 AND l.city = $2
                "#,
                vec![state.clone().into(), c.clone().into()],
            ),
            None => (
                r#"
                    SELECT
                        MAX(l.lat) as north_east_lat,
                        MAX(l.long) as north_east_long,
                        MIN(l.lat) as south_west_lat,
                        MIN(l.long) as south_west_long
                    FROM locations l
                    INNER JOIN artists a ON a.location_id = l.id
                    WHERE l.state = $1
                "#,
                vec![state.into()],
            ),
        };

        BoundingBox::find_by_statement(Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            query,
            params,
        ))
        .one(&self.db)
        .await
    }
}
