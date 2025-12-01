use poem_openapi::Object;
use sea_orm::FromQueryResult;

#[derive(Debug, FromQueryResult, Object)]
pub struct PostsCount {
    pub count: i64,
}

#[derive(Debug, Object)]
pub struct PaginatedPosts {
    pub posts: Vec<TattooPost>,
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
}

#[derive(Debug, FromQueryResult, Object)]
pub struct TattooPost {
    pub id: i64,
    pub url: String,
    pub artist_id: i64,
    pub artist_name: String,
    pub location_id: Option<i64>,
    pub location_name: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub is_favorited: bool,
}
