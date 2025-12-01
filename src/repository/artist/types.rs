use poem_openapi::Object;
use sea_orm::FromQueryResult;

#[derive(Debug, FromQueryResult, Object)]
pub struct ArtistData {
    pub id: i64,
    pub name: String,
    pub years_experience: Option<i32>,
    pub instagram_handle: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub is_subscribed: bool,
}

#[derive(Debug, Object)]
pub struct ArtistWithDetails {
    pub artist: ArtistData,
    pub location: Option<ArtistLocation>,
    pub styles: Vec<ArtistStyle>,
}

#[derive(Debug, FromQueryResult, Object)]
pub struct ArtistLocation {
    pub id: i64,
    pub name: String,
    pub city: Option<String>,
    pub state: Option<String>,
    pub address: Option<String>,
}

#[derive(Debug, FromQueryResult, Object)]
pub struct ArtistStyle {
    pub id: i64,
    pub name: String,
    pub count: i64,
}

#[derive(Debug, Object)]
pub struct PaginatedArtistImages {
    pub images: Vec<ArtistImage>,
    pub total: i64,
    pub page: i32,
    pub per_page: i32,
}

#[derive(Debug, FromQueryResult, Object)]
pub struct ArtistImage {
    pub id: i64,
    pub url: String,
    pub is_favorited: bool,
}
