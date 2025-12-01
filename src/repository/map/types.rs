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
