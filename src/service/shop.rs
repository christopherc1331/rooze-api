use crate::{error::AppError, repository::shop::ShopRepository};

pub struct ShopService {
    repo: ShopRepository,
}

impl ShopService {
    pub fn new(repo: ShopRepository) -> Self {
        Self { repo }
    }

    pub async fn get_shop(
        &self,
        location_id: i64,
        user_id: Option<i64>,
    ) -> Result<Option<crate::repository::shop::types::ShopWithDetails>, AppError> {
        Ok(self.repo.get_shop(location_id, user_id).await?)
    }

    pub async fn get_shop_filtered_styles(
        &self,
        location_id: i64,
        selected_style_ids: Option<Vec<i64>>,
    ) -> Result<Vec<crate::repository::shop::types::ShopStyle>, AppError> {
        Ok(self
            .repo
            .get_shop_filtered_styles(location_id, selected_style_ids)
            .await?)
    }

    pub async fn get_shop_images_paginated(
        &self,
        location_id: i64,
        style_ids: Option<Vec<i64>>,
        page: i32,
        per_page: i32,
        user_id: Option<i64>,
    ) -> Result<crate::repository::shop::types::PaginatedShopImages, AppError> {
        Ok(self
            .repo
            .get_shop_images_paginated(location_id, style_ids, page, per_page, user_id)
            .await?)
    }
}
