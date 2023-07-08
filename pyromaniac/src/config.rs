use std::path::PathBuf;

use anyhow::Result;
use dotenvy::dotenv;
use std::sync::OnceLock;

#[derive(Debug)]
pub struct Config {
    pub resource_path: PathBuf,
}

static CONFIG: OnceLock<Config> = OnceLock::new();

impl Config {
    fn init_from_env() -> Result<Config> {
        //load dotenv file if it exists
        match dotenv() {
            Err(_) => tracing::info!("No .env file found, nothing to load"),
            Ok(_) => tracing::info!("Loaded config from .env file"),
        }

        let resource_path = dotenvy::var("RESOURCE_PATH")
            .map_err(Into::<anyhow::Error>::into) //error trait bullshit
            .unwrap_or_else(|_| {
                tracing::warn!("No resource path provided defaulting to ./resources");
                "./resources".to_owned()
            })
            .into();

        let c = Ok(Config { resource_path });

        tracing::info!("Loaded config from environment!");

        c
    }
}
pub fn get() -> &'static Config {
    CONFIG.get_or_init(|| Config::init_from_env().unwrap())
}