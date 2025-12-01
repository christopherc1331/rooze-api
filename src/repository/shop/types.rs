use poem_openapi::Object;
use sea_orm::FromQueryResult;

#[derive(Debug, FromQueryResult, Object)]
pub struct ShopData {
    pub id: i64,
    pub name: String,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub website: Option<String>,
    pub lat: Option<f64>,
    pub long: Option<f64>,
}

#[derive(Debug, Object)]
pub struct ShopWithDetails {
    pub shop: ShopData,
    pub artists: Vec<ShopArtist>,
    pub styles: Vec<ShopStyle>,
}

#[derive(Debug, FromQueryResult, Object)]
pub struct ShopArtist {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, FromQueryResult, Object)]
pub struct ShopStyle {
    pub id: i64,
    pub name: String,
    pub count: i64,
}

#[derive(Debug, Object)]
pub struct PaginatedShopImages {
    pub images: Vec<ShopImage>,
    pub total: i64,
    pub page: i32,
    pub per_page: i32,
}

#[derive(Debug, FromQueryResult, Object)]
pub struct ShopImage {
    pub id: i64,
    pub url: String,
    pub artist_id: i64,
    pub artist_name: String,
    pub is_favorited: bool,
}
