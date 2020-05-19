use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub skyway: SkywayConfig,
}

#[derive(Deserialize)]
pub struct SkywayConfig {
    pub key: String,
}
