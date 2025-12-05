use poem_openapi::{
    OpenApi,
    param::{Header, Query},
    payload::Json,
};

use crate::{
    error::ErrorResponse,
    repository::posts::{
        PostsRepository,
        types::{PaginatedPosts, PostsCount},
    },
    service::PostsService,
};

pub struct PostsApi {
    service: PostsService,
}

#[OpenApi(prefix_path = "/posts")]
impl PostsApi {
    pub fn new(state: std::sync::Arc<crate::AppState>) -> Self {
        let repo = PostsRepository::new(state.db.clone());
        let service = PostsService::new(repo);
        Self { service }
    }

    #[oai(path = "/count", method = "get")]
    async fn get_posts_count(
        &self,
        styles: Query<String>,
        states: Query<Option<String>>,
        cities: Query<Option<String>>,
    ) -> Result<Json<PostsCount>, ErrorResponse> {
        let styles_parsed: Vec<String> = styles
            .0
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        let states_parsed = states.0.map(|s| {
            s.split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect()
        });
        let cities_parsed = cities.0.map(|s| {
            s.split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect()
        });
        let count = self
            .service
            .get_posts_count(styles_parsed, states_parsed, cities_parsed)
            .await
            .map_err(ErrorResponse::from)?;
        Ok(Json(count))
    }

    #[oai(path = "/", method = "get")]
    async fn get_posts(
        &self,
        styles: Query<String>,
        states: Query<Option<String>>,
        cities: Query<Option<String>>,
        limit: Query<Option<i64>>,
        offset: Query<Option<i64>>,
        #[oai(name = "Authorization")] token: Header<Option<String>>,
    ) -> Result<Json<PaginatedPosts>, ErrorResponse> {
        let styles_parsed: Vec<String> = styles
            .0
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        let states_parsed = states.0.map(|s| {
            s.split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect()
        });
        let cities_parsed = cities.0.map(|s| {
            s.split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect()
        });
        // TODO: Extract user_id from token
        let user_id = token
            .0
            .as_ref()
            .map(|_| todo!("extract user_id from token"));
        let posts = self
            .service
            .get_posts_by_style(
                styles_parsed,
                states_parsed,
                cities_parsed,
                limit.0.unwrap_or(20),
                offset.0.unwrap_or(0),
                user_id,
            )
            .await
            .map_err(ErrorResponse::from)?;
        Ok(Json(posts))
    }
}
