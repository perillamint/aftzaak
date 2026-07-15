use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Http {
    pub listen: std::net::IpAddr,
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Db {
    pub database_url: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Jwt {
    pub secret: String,
    pub ttl: i64,
    pub issuer: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub http: Http,
    pub db: Db,
    pub jwt: Jwt,
}

impl Config {
    pub async fn parse_config_file(file: &str) -> Result<Self, anyhow::Error> {
        let config_str = tokio::fs::read_to_string(file).await?;
        Ok(toml::from_str(&config_str)?)
    }
}
