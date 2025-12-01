use poem_openapi::{
    OpenApi,
    param::{Header, Path, Query},
    payload::Json,
};

use crate::{
    error::ErrorResponse,
    repository::artist::{
        ArtistRepository,
        types::{ArtistStyle, ArtistWithDetails, PaginatedArtistImages},
    },
    service::ArtistService,
};

pub struct ArtistApi {
    service: ArtistService,
}

#[OpenApi(prefix_path = "/artists")]
impl ArtistApi {
    pub fn new(state: std::sync::Arc<crate::AppState>) -> Self {
        let repo = ArtistRepository::new(state.db.clone());
        let service = ArtistService::new(repo);
        Self { service }
    }

    /// Get artist profile with location and styles
    #[oai(path = "/:id", method = "get")]
    async fn get_artist(
        &self,
        id: Path<i64>,
    ) -> Result<Json<Option<ArtistWithDetails>>, ErrorResponse> {
        let artist = self
            .service
            .get_artist(id.0)
            .await
            .map_err(ErrorResponse::from)?;
        Ok(Json(artist))
    }

    /// Get styles filtered by selected styles (co-occurrence) for an artist
    #[oai(path = "/:id/styles", method = "get")]
    async fn get_artist_filtered_styles(
        &self,
        id: Path<i64>,
        style_ids: Query<Option<String>>,
    ) -> Result<Json<Vec<ArtistStyle>>, ErrorResponse> {
        let style_ids_parsed = style_ids.0.map(|s| {
            s.split(',')
                .filter_map(|id| id.trim().parse::<i64>().ok())
                .collect()
        });
        let styles = self
            .service
            .get_artist_filtered_styles(id.0, style_ids_parsed)
            .await
            .map_err(ErrorResponse::from)?;
        Ok(Json(styles))
    }

    /// Get paginated images for an artist with optional style filtering
    #[oai(path = "/:id/images", method = "get")]
    async fn get_artist_images(
        &self,
        id: Path<i64>,
        style_ids: Query<Option<String>>,
        page: Query<Option<i32>>,
        per_page: Query<Option<i32>>,
        #[oai(name = "Authorization")] token: Header<Option<String>>,
    ) -> Result<Json<PaginatedArtistImages>, ErrorResponse> {
        let style_ids_parsed = style_ids.0.map(|s| {
            s.split(',')
                .filter_map(|id| id.trim().parse::<i64>().ok())
                .collect()
        });
        // TODO: Extract user_id from token
        let user_id = token
            .0
            .as_ref()
            .map(|_| todo!("extract user_id from token"));
        let images = self
            .service
            .get_artist_images_paginated(
                id.0,
                style_ids_parsed,
                page.0.unwrap_or(1),
                per_page.0.unwrap_or(20),
                user_id,
            )
            .await
            .map_err(ErrorResponse::from)?;
        Ok(Json(images))
    }
}
