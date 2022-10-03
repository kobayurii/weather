use crate::config::{Command, Config, Opts, WEATHER_CONFIG_NAME};
use crate::providers::{Providers, WeatherInfo};
use clap::Parser;
use providers::accuweather::AccuWeather;
use providers::openweather::OpenWeather;

mod config;
mod providers;

async fn get_accu_weather(cfg: Config) -> anyhow::Result<WeatherInfo> {
    let client = AccuWeather::new(cfg.api_key, cfg.lat, cfg.lon);
    let weather = client.get_current_weather().await?;
    Ok(weather)
}

async fn get_open_weather(cfg: Config) -> anyhow::Result<WeatherInfo> {
    let client = OpenWeather::new(cfg.api_key, cfg.lat, cfg.lon);
    let weather = client.get_current_weather().await?;
    Ok(weather)
}

async fn get_weather() -> anyhow::Result<WeatherInfo> {
    let cfg: Config = confy::load(WEATHER_CONFIG_NAME)?;
    let weather_info = match cfg.provider {
        Providers::OpenWeather => get_open_weather(cfg.clone()).await?,
        Providers::AccuWeather => get_accu_weather(cfg.clone()).await?,
    };
    Ok(weather_info)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts: Opts = Opts::parse();
    match opts.command {
        Command::Config => match opts.run_config().await {
            Ok(_) => println!("Configuration save successful"),
            Err(e) => println!("Error to save configuration: {:?}", e),
        },
        Command::Get => match get_weather().await {
            Ok(weather_info) => println!("{}", weather_info),
            Err(e) => println!("Error to get configuration: {:?}", e),
        },
    }
    Ok(())
}
