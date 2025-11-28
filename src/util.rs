pub fn get_env_var(key: &str, default: &str) -> String {
    dotenv::var(key).unwrap_or(default.to_string())
}
