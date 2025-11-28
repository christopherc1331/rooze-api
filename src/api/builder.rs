use std::sync::Arc;

use poem_openapi::OpenApi;

use crate::AppState;

pub trait ApiModule: Sized {
    type Api: OpenApi;

    fn build(state: Arc<AppState>) -> Self::Api;
}
