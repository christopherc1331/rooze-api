use sea_orm::{DatabaseConnection, DbErr};

use super::types::{ArtistStyle, ArtistWithDetails, PaginatedArtistImages};

pub struct ArtistRepository {
    #[allow(dead_code)]
    db: DatabaseConnection,
}

impl ArtistRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn get_artist(&self, _artist_id: i64) -> Result<Option<ArtistWithDetails>, DbErr> {
        todo!()
    }

    pub async fn get_artist_filtered_styles(
        &self,
        _artist_id: i64,
        _selected_style_ids: Option<Vec<i64>>,
    ) -> Result<Vec<ArtistStyle>, DbErr> {
        todo!()
    }

    pub async fn get_artist_images_paginated(
        &self,
        _artist_id: i64,
        _style_ids: Option<Vec<i64>>,
        _page: i32,
        _per_page: i32,
        _user_id: Option<i64>,
    ) -> Result<PaginatedArtistImages, DbErr> {
        todo!()
    }
}
