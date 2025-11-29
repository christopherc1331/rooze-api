use poem_openapi::{ApiResponse, payload::PlainText};

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sea_orm::DbErr),
}

#[derive(ApiResponse)]
pub enum ErrorResponse {
    #[oai(status = 500)]
    InternalServerError(PlainText<String>),
}

impl From<AppError> for ErrorResponse {
    fn from(err: AppError) -> Self {
        match err {
            AppError::Database(e) => ErrorResponse::InternalServerError(PlainText(e.to_string())),
        }
    }
}
