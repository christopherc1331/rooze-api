use sea_orm::{DatabaseConnection, DbErr, FromQueryResult, Statement};
use std::collections::HashMap;

use super::types::{PaginatedShopImages, ShopArtist, ShopData, ShopStyle, ShopWithDetails};
use crate::repository::map::types::GeoBoundary;

type ArtistStylesMap = HashMap<i64, String>;
type ArtistDataMap = HashMap<i64, (String, ArtistStylesMap)>;
type ShopArtistsMap = HashMap<i64, ArtistDataMap>;

pub struct ShopRepository {
    #[allow(dead_code)]
    db: DatabaseConnection,
}

impl ShopRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn get_shop(
        &self,
        _location_id: i64,
        _user_id: Option<i64>,
    ) -> Result<Option<ShopWithDetails>, DbErr> {
        todo!()
    }

    pub async fn get_shops_in_bounds(
        &self,
        boundary: GeoBoundary,
        style_ids: Option<Vec<i64>>,
    ) -> Result<Vec<ShopWithDetails>, DbErr> {
        let GeoBoundary {
            south_west_lat,
            north_east_lat,
            south_west_long,
            north_east_long,
        } = boundary;

        #[derive(Debug, FromQueryResult)]
        struct ShopArtistStyleRow {
            location_id: i64,
            location_name: String,
            address: Option<String>,
            city: Option<String>,
            state: Option<String>,
            postal_code: Option<String>,
            website_uri: Option<String>,
            lat: Option<f32>,
            long: Option<f32>,
            artist_id: i64,
            artist_name: String,
            style_id: Option<i64>,
            style_name: Option<String>,
        }

        let (query, params): (String, Vec<sea_orm::Value>) = match &style_ids {
            Some(ids) if !ids.is_empty() => (
                format!(
                    r#"
                    WITH locations_in_bounds AS (
                        SELECT id, name, address, city, state, postal_code, website_uri, lat, long
                        FROM locations
                        WHERE lat BETWEEN $1 AND $2
                        AND long BETWEEN $3 AND $4
                    ),
                    matching_artists AS (
                        SELECT DISTINCT ai.artist_id
                        FROM artists_images ai
                        INNER JOIN artists_images_styles ais ON ai.id = ais.artists_images_id
                        WHERE ais.style_id = ANY($5)
                        GROUP BY ai.id, ai.artist_id
                        HAVING COUNT(DISTINCT ais.style_id) = {}
                    ),
                    artist_style_counts AS (
                        SELECT
                            ai.artist_id,
                            ais.style_id,
                            COUNT(*) as image_count
                        FROM artists_images ai
                        INNER JOIN artists_images_styles ais ON ai.id = ais.artists_images_id
                        WHERE ai.active IS NOT FALSE
                        GROUP BY ai.artist_id, ais.style_id
                    )
                    SELECT
                        l.id as location_id,
                        l.name as location_name,
                        l.address,
                        l.city,
                        l.state,
                        l.postal_code,
                        l.website_uri,
                        l.lat,
                        l.long,
                        a.id as artist_id,
                        a.name as artist_name,
                        s.id as style_id,
                        s.name as style_name
                    FROM locations_in_bounds l
                    INNER JOIN artists a ON a.location_id = l.id
                    INNER JOIN matching_artists ma ON a.id = ma.artist_id
                    LEFT JOIN artist_style_counts sc ON a.id = sc.artist_id
                    LEFT JOIN styles s ON sc.style_id = s.id
                    ORDER BY sc.image_count DESC
                "#,
                    ids.len()
                ),
                vec![
                    south_west_lat.into(),
                    north_east_lat.into(),
                    south_west_long.into(),
                    north_east_long.into(),
                    ids.clone().into(),
                ],
            ),
            _ => (
                r#"
                    WITH locations_in_bounds AS (
                        SELECT id, name, address, city, state, postal_code, website_uri, lat, long
                        FROM locations
                        WHERE lat BETWEEN $1 AND $2
                        AND long BETWEEN $3 AND $4
                    ),
                    artist_style_counts AS (
                        SELECT
                            ai.artist_id,
                            ais.style_id,
                            COUNT(*) as image_count
                        FROM artists_images ai
                        INNER JOIN artists_images_styles ais ON ai.id = ais.artists_images_id
                        WHERE ai.active IS NOT FALSE
                        GROUP BY ai.artist_id, ais.style_id
                    )
                    SELECT
                        l.id as location_id,
                        l.name as location_name,
                        l.address,
                        l.city,
                        l.state,
                        l.postal_code,
                        l.website_uri,
                        l.lat,
                        l.long,
                        a.id as artist_id,
                        a.name as artist_name,
                        s.id as style_id,
                        s.name as style_name
                    FROM locations_in_bounds l
                    INNER JOIN artists a ON a.location_id = l.id
                    LEFT JOIN artist_style_counts sc ON a.id = sc.artist_id
                    LEFT JOIN styles s ON sc.style_id = s.id
                    ORDER BY sc.image_count DESC
                "#
                .to_string(),
                vec![
                    south_west_lat.into(),
                    north_east_lat.into(),
                    south_west_long.into(),
                    north_east_long.into(),
                ],
            ),
        };

        let rows = ShopArtistStyleRow::find_by_statement(Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            &query,
            params,
        ))
        .all(&self.db)
        .await?;

        if rows.is_empty() {
            return Ok(vec![]);
        }

        let mut shops_map: HashMap<i64, ShopData> = HashMap::new();
        let mut artists_by_shop: ShopArtistsMap = HashMap::new();

        for row in rows {
            shops_map
                .entry(row.location_id)
                .or_insert_with(|| ShopData {
                    id: row.location_id,
                    name: row.location_name.clone(),
                    address: row.address.clone(),
                    city: row.city.clone(),
                    state: row.state.clone(),
                    postal_code: row.postal_code.clone(),
                    website: row.website_uri.clone(),
                    lat: row.lat,
                    long: row.long,
                });

            let artist_entry = artists_by_shop
                .entry(row.location_id)
                .or_default()
                .entry(row.artist_id)
                .or_insert_with(|| (row.artist_name.clone(), HashMap::new()));

            if let (Some(style_id), Some(style_name)) = (row.style_id, row.style_name) {
                artist_entry.1.entry(style_id).or_insert(style_name);
            }
        }

        let result = shops_map
            .into_iter()
            .map(|(shop_id, shop)| {
                let artists: Vec<ShopArtist> = artists_by_shop
                    .remove(&shop_id)
                    .map(|artists_map| {
                        artists_map
                            .into_iter()
                            .map(|(artist_id, (artist_name, styles_map))| {
                                let styles: Vec<ShopStyle> = styles_map
                                    .into_iter()
                                    .map(|(id, name)| ShopStyle { id, name })
                                    .collect();
                                ShopArtist {
                                    id: artist_id,
                                    name: artist_name,
                                    styles,
                                }
                            })
                            .collect()
                    })
                    .unwrap_or_default();

                ShopWithDetails { shop, artists }
            })
            .collect();

        Ok(result)
    }

    pub async fn get_shop_filtered_styles(
        &self,
        _location_id: i64,
        _selected_style_ids: Option<Vec<i64>>,
    ) -> Result<Vec<ShopStyle>, DbErr> {
        todo!()
    }

    pub async fn get_shop_images_paginated(
        &self,
        _location_id: i64,
        _style_ids: Option<Vec<i64>>,
        _page: i32,
        _per_page: i32,
        _user_id: Option<i64>,
    ) -> Result<PaginatedShopImages, DbErr> {
        todo!()
    }
}
