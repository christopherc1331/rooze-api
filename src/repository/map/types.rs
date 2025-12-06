use poem_openapi::Object;
use sea_orm::FromQueryResult;

#[derive(Debug, FromQueryResult, Object)]
pub struct MapStats {
    pub shop_count: i64,
    pub artist_count: i64,
    pub style_count: i64,
}

#[derive(Debug, Clone, Copy)]
pub struct GeoBoundary {
    pub north_east_lat: f64,
    pub north_east_long: f64,
    pub south_west_lat: f64,
    pub south_west_long: f64,
}

#[derive(Debug, FromQueryResult, Object)]
pub struct City {
    pub name: String,
    pub state: String,
    pub lat: f64,
    pub long: f64,
}

#[derive(Debug, FromQueryResult, Object)]
pub struct State {
    pub name: String,
    pub artist_count: i64,
}

#[derive(Debug, FromQueryResult, Object)]
pub struct LocationWithDetails {
    pub id: i64,
    pub name: String,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub lat: f64,
    pub long: f64,
    pub artist_count: i64,
}

#[derive(Debug, FromQueryResult, Object)]
pub struct PostalCodeResult {
    pub lat: f64,
    pub long: f64,
    pub city: Option<String>,
    pub state: Option<String>,
}

#[derive(Debug, FromQueryResult, Object)]
pub struct BoundingBox {
    pub north_east_lat: Option<f32>,
    pub north_east_long: Option<f32>,
    pub south_west_lat: Option<f32>,
    pub south_west_long: Option<f32>,
}
