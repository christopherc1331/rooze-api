use poem_openapi::Object;
use sea_orm::FromQueryResult;

#[derive(Debug, FromQueryResult, Object)]
pub struct MapState {
    pub shop_count: i64,
    pub artist_count: i64,
    pub style_count: i64,
}

pub struct GeoBoundary {
    pub north_east_lat: f64,
    pub north_east_long: f64,
    pub south_west_lat: f64,
    pub south_west_long: f64,
}
