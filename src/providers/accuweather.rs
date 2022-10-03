use crate::providers::{WeatherInfo, WeatherLocation};
use serde::{Deserialize, Serialize};
use serde_json::Value;

const BASE_URL: &str = "http://dataservice.accuweather.com/";

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct AccuWeather {
    api_key: String,
    lat: String,
    lon: String,
}

impl AccuWeather {
    pub(crate) fn new(api_key: String, lat: String, lon: String) -> Self {
        Self { api_key, lat, lon }
    }

    pub(crate) async fn get_location(&self) -> anyhow::Result<WeatherLocation> {
        let url = format!(
            "{}locations/v1/cities/geoposition/search?apikey={}&q={}%2C{}",
            BASE_URL,
            self.api_key.clone(),
            self.lat.clone(),
            self.lon.clone()
        );
        let response = reqwest::get(&url).await?;
        let body = response.text().await?;
        let data: Value = serde_json::from_str(&body)?;
        Ok(WeatherLocation {
            key: data["Key"]
                .as_str()
                .expect("Error to parse location key")
                .to_owned(),
            name: data["AdministrativeArea"]["EnglishName"]
                .as_str()
                .expect("Error to parse location name")
                .to_owned(),
        })
    }

    pub(crate) async fn get_current_weather(&self) -> anyhow::Result<WeatherInfo> {
        let location = self.get_location().await?;
        let url = format!(
            "{}currentconditions/v1/{}?apikey={}&language=en-us&details=true",
            BASE_URL,
            location.key.clone(),
            self.api_key.clone()
        );
        let response = reqwest::get(&url).await?;
        let body = response.text().await?;
        let data: Value = serde_json::from_str(&body)?;
        let mut weather_info = WeatherInfo::default();
        let weather_data = data.as_array().expect("Error to parse weather data")[0]
            .as_object()
            .expect("Error to parse weather data object");
        weather_info.city = location.name.to_owned();
        weather_info.weather = weather_data["WeatherText"]
            .as_str()
            .expect("Error to parse weather text")
            .to_owned();
        weather_info.temp = weather_data["Temperature"]["Metric"]["Value"]
            .as_f64()
            .expect("Error to parse temp");
        weather_info.temp_feels_like = weather_data["RealFeelTemperature"]["Metric"]["Value"]
            .as_f64()
            .expect("Error to parse temp_feels_like");
        weather_info.pressure = weather_data["Pressure"]["Metric"]["Value"]
            .as_f64()
            .expect("Error to parse pressure");
        weather_info.humidity = weather_data["RelativeHumidity"]
            .as_f64()
            .expect("Error to parse humidity");
        weather_info.wind_speed = weather_data["Wind"]["Speed"]["Metric"]["Value"]
            .as_f64()
            .expect("Error to parse wind speed")
            * 1000.0
            / 3600.0;
        weather_info.wind_degree = weather_data["Wind"]["Direction"]["Degrees"]
            .as_f64()
            .expect("Error to parse wind deg");
        Ok(weather_info)
    }
}
