use crate::providers::Providers;
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io;
use strum::IntoEnumIterator;

pub const WEATHER_CONFIG_NAME: &str = "weatherconfig";

#[derive(Parser, Debug)]
#[clap(
    version,
    author,
    about,
    setting(clap::AppSettings::DisableHelpSubcommand),
    setting(clap::AppSettings::PropagateVersion),
    setting(clap::AppSettings::NextLineHelp)
)]
pub struct Opts {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Command {
    Config,
    Get,
}

impl Opts {
    fn read_provider() -> Providers {
        let mut providers_map = HashMap::new();
        for (i, provider) in Providers::iter().enumerate() {
            providers_map.insert(i.to_string(), provider);
        }
        println!("Select a provider:");
        for (key, value) in &providers_map {
            println!("{:?} - {:?}", key, value);
        }

        loop {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            match providers_map.get(&input.trim().to_string()) {
                Some(provider) => return provider.clone(),
                None => println!("Invalid provider"),
            }
        }
    }

    fn read_config(config_name: &str) -> String {
        let mut input = String::new();
        println!("Enter {}:", config_name);
        loop {
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            if !input.trim().to_string().is_empty() {
                return input.trim().to_string();
            } else {
                println!("Invalid input")
            }
        }
    }

    pub(crate) async fn run_config(&self) -> anyhow::Result<Config> {
        let mut weather_cfg = Config::new();
        weather_cfg.provider = Self::read_provider();
        weather_cfg.api_key = Self::read_config("KEY_API");
        weather_cfg.lat = Self::read_config("Latitude");
        weather_cfg.lon = Self::read_config("Longitude");
        confy::store(WEATHER_CONFIG_NAME, weather_cfg.clone())?;
        Ok(weather_cfg)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Config {
    pub provider: Providers,
    pub api_key: String,
    pub lat: String,
    pub lon: String,
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }
}
