use poem_openapi::Object;
use sea_orm::FromQueryResult;

#[derive(Debug, FromQueryResult, Object)]
pub struct StyleWithCount {
    pub id: i64,
    pub name: String,
    pub count: i64,
}

#[derive(Debug, FromQueryResult, Object)]
pub struct StyleTypeWithCount {
    pub id: i64,
    pub name: String,
    pub count: i64,
    pub style_type: String,
}

pub struct GeoBoundary {
    pub north_east_lat: f64,
    pub north_east_long: f64,
    pub south_west_lat: f64,
    pub south_west_long: f64,
}
