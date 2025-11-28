pub fn get_popular_styles(limit: Option<usize>) -> Vec<String> {
    let styles = vec![
        "Style A".to_string(),
        "Style B".to_string(),
        "Style C".to_string(),
        "Style D".to_string(),
        "Style E".to_string(),
    ];
    match limit {
        Some(l) => styles.into_iter().take(l).collect(),
        None => styles,
    }
}
