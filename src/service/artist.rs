use crate::{error::AppError, repository::artist::ArtistRepository};

pub struct ArtistService {
    repo: ArtistRepository,
}

impl ArtistService {
    pub fn new(repo: ArtistRepository) -> Self {
        Self { repo }
    }

    pub async fn get_artist(
        &self,
        artist_id: i64,
    ) -> Result<Option<crate::repository::artist::types::ArtistWithDetails>, AppError> {
        Ok(self.repo.get_artist(artist_id).await?)
    }

    pub async fn get_artist_filtered_styles(
        &self,
        artist_id: i64,
        selected_style_ids: Option<Vec<i64>>,
    ) -> Result<Vec<crate::repository::artist::types::ArtistStyle>, AppError> {
        Ok(self
            .repo
            .get_artist_filtered_styles(artist_id, selected_style_ids)
            .await?)
    }

    pub async fn get_artist_images_paginated(
        &self,
        artist_id: i64,
        style_ids: Option<Vec<i64>>,
        page: i32,
        per_page: i32,
        user_id: Option<i64>,
    ) -> Result<crate::repository::artist::types::PaginatedArtistImages, AppError> {
        Ok(self
            .repo
            .get_artist_images_paginated(artist_id, style_ids, page, per_page, user_id)
            .await?)
    }
}
