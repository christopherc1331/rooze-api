use sea_orm::{DatabaseConnection, DbErr};

use super::types::{PaginatedShopImages, ShopStyle, ShopWithDetails};

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
