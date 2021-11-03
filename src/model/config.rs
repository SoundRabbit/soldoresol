use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub skyway: SkywayConfig,
    pub client: ClientConfig,
    pub drive: DriveConfig,
}

#[derive(Deserialize)]
pub struct SkywayConfig {
    pub key: String,
}

#[derive(Deserialize)]
pub struct ClientConfig {
    pub db_prefix: String,
}

#[derive(Deserialize)]
pub struct DriveConfig {
    pub api_key: String,
    pub client_id: String,
}
