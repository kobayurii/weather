use crate::providers::WeatherInfo;
use serde::{Deserialize, Serialize};
use serde_json::Value;

const BASE_URL: &str = "http://api.openweathermap.org/";

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct OpenWeather {
    api_key: String,
    lat: String,
    lon: String,
}

impl OpenWeather {
    pub(crate) fn new(api_key: String, lat: String, lon: String) -> Self {
        Self { api_key, lat, lon }
    }

    pub(crate) async fn get_current_weather(&self) -> anyhow::Result<WeatherInfo> {
        let url = format!(
            "{}data/2.5/weather?lat={}&lon={}&units=metric&appid={}",
            BASE_URL,
            self.lat.clone(),
            self.lon.clone(),
            self.api_key.clone()
        );
        let response = reqwest::get(&url).await?;
        let body = response.text().await?;
        let data: Value = serde_json::from_str(&body)?;
        let weather_stats = data["main"]
            .as_object()
            .expect("Error to parse weather status");
        let wind_stats = data["wind"]
            .as_object()
            .expect("Error to parse wind status");
        Ok(WeatherInfo {
            city: data["name"]
                .as_str()
                .expect("Error to parse city")
                .to_owned(),
            weather: data["weather"]
                .as_array()
                .expect("Parse error weather object")[0]
                .as_object()
                .expect("Error to parse main info")["main"]
                .as_str()
                .expect("Main info parse error")
                .to_owned(),

            temp: weather_stats["temp"].as_f64().expect("Error to parse temp"),
            temp_feels_like: weather_stats["feels_like"]
                .as_f64()
                .expect("Error to parse temp_feels_like"),
            pressure: weather_stats["pressure"]
                .as_f64()
                .expect("Error to parse pressure"),
            humidity: weather_stats["humidity"]
                .as_f64()
                .expect("Error to parse humidity"),

            wind_speed: wind_stats["speed"]
                .as_f64()
                .expect("Error to parse wind speed"),
            wind_degree: wind_stats["deg"].as_f64().expect("Error to parse wind deg"),
        })
    }
}
