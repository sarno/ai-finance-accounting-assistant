use std::env;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub api_port: u16,
    pub jwt_secret: String,
    pub jwt_access_minutes: i64,
    pub jwt_refresh_days: i64,
    pub ai_service_token: String,
    pub storage_base_path: String,
    pub storage_base_url: String,
}

impl AppConfig {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            database_url: required_env("DATABASE_URL")?,
            api_port: env::var("API_PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()?,
            jwt_secret: required_env("JWT_SECRET")?,
            jwt_access_minutes: env::var("JWT_ACCESS_TOKEN_MINUTES")
                .unwrap_or_else(|_| "30".to_string())
                .parse()?,
            jwt_refresh_days: env::var("JWT_REFRESH_TOKEN_DAYS")
                .unwrap_or_else(|_| "14".to_string())
                .parse()?,
            ai_service_token: required_env("AI_SERVICE_TOKEN")?,
            storage_base_path: env::var("STORAGE_BASE_PATH")
                .unwrap_or_else(|_| "/data/uploads".to_string()),
            storage_base_url: env::var("STORAGE_BASE_URL")
                .unwrap_or_else(|_| "http://localhost:8080/uploads".to_string()),
        })
    }
}

fn required_env(key: &str) -> anyhow::Result<String> {
    env::var(key).map_err(|_| anyhow::anyhow!("Required environment variable '{}' is not set", key))
}
