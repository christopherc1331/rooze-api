use crate::repository::styles_repository;

pub fn get_popular_styles(limit: usize) -> Vec<String> {
    styles_repository::get_popular_styles(Some(limit))
}
