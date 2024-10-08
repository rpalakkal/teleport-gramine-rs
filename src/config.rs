use std::path::PathBuf;

use figment::{
    providers::{Env, Format, Toml},
    Figment,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub paths: PathConfig,

    // TODO: add proper nested config structure for env vars
    pub ws_rpc_url: String,
    pub rpc_url: String,
    pub tee_url: String,
    pub rpc_key: String,
    pub nft_minter_mnemonic: String,
    pub db_path: String,
    pub app_url: String,
    pub database_url: String,
    pub twitter_consumer_key: String,
    pub twitter_consumer_secret: String,
}

#[derive(Debug, Deserialize)]
pub struct PathConfig {
    pub private_key: PathBuf,
    pub certificate: PathBuf,
    pub csr: PathBuf,
    pub quote: PathBuf,
}

impl Config {
    pub fn new() -> Result<Self, figment::Error> {
        dotenv::dotenv().ok();
        dotenv::from_filename("/teleport.env").ok();

        Figment::new().merge(Toml::file("config/default.toml").nested()).merge(Env::raw()).extract()
    }
}