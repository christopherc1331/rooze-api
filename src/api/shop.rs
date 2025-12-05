use poem_openapi::{
    OpenApi,
    param::{Header, Path, Query},
    payload::Json,
};

use crate::{
    error::ErrorResponse,
    repository::shop::{
        ShopRepository,
        types::{PaginatedShopImages, ShopStyle, ShopWithDetails},
    },
    service::ShopService,
};

pub struct ShopApi {
    service: ShopService,
}

#[OpenApi(prefix_path = "/shops")]
impl ShopApi {
    pub fn new(state: std::sync::Arc<crate::AppState>) -> Self {
        let repo = ShopRepository::new(state.db.clone());
        let service = ShopService::new(repo);
        Self { service }
    }

    #[oai(path = "/:id", method = "get")]
    async fn get_shop(
        &self,
        id: Path<i64>,
        #[oai(name = "Authorization")] token: Header<Option<String>>,
    ) -> Result<Json<Option<ShopWithDetails>>, ErrorResponse> {
        // TODO: Extract user_id from token
        let user_id = token
            .0
            .as_ref()
            .map(|_| todo!("extract user_id from token"));
        let shop = self
            .service
            .get_shop(id.0, user_id)
            .await
            .map_err(ErrorResponse::from)?;
        Ok(Json(shop))
    }

    #[oai(path = "/:id/styles", method = "get")]
    async fn get_shop_filtered_styles(
        &self,
        id: Path<i64>,
        style_ids: Query<Option<String>>,
    ) -> Result<Json<Vec<ShopStyle>>, ErrorResponse> {
        let style_ids_parsed = style_ids.0.map(|s| {
            s.split(',')
                .filter_map(|id| id.trim().parse::<i64>().ok())
                .collect()
        });
        let styles = self
            .service
            .get_shop_filtered_styles(id.0, style_ids_parsed)
            .await
            .map_err(ErrorResponse::from)?;
        Ok(Json(styles))
    }

    #[oai(path = "/:id/images", method = "get")]
    async fn get_shop_images(
        &self,
        id: Path<i64>,
        style_ids: Query<Option<String>>,
        page: Query<Option<i32>>,
        per_page: Query<Option<i32>>,
        #[oai(name = "Authorization")] token: Header<Option<String>>,
    ) -> Result<Json<PaginatedShopImages>, ErrorResponse> {
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
            .get_shop_images_paginated(
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
