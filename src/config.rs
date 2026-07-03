use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Http {
    pub host: String,
    pub port: u16,
}

#[derive(Deserialize, Serialize)]
pub struct Db {
    pub database_url: String,
}

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub http: Http,
    pub db: Db,
}

impl Config {
    pub async fn parse_config_file(file: &str) -> Result<Self, anyhow::Error> {
        let config_str = tokio::fs::read_to_string(file).await?;
        Ok(toml::from_str(&config_str)?)
    }
}
