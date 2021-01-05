use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub skyway: SkywayConfig,
    pub client: ClientConfig,
}

#[derive(Deserialize)]
pub struct SkywayConfig {
    pub key: String,
}

#[derive(Deserialize)]
pub struct ClientConfig {
    pub db_prefix: String,
}
