use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result};
use strum_macros::EnumIter;

pub mod accuweather;
pub mod openweather;

#[derive(Serialize, Deserialize, Debug, Clone, EnumIter)]
pub enum Providers {
    AccuWeather,
    OpenWeather,
}

impl Default for Providers {
    fn default() -> Self {
        Self::AccuWeather
    }
}

pub struct WeatherLocation {
    key: String,
    name: String,
}

pub struct WeatherInfo {
    pub city: String,
    pub weather: String,
    pub temp: f64,
    pub temp_feels_like: f64,
    pub pressure: f64,
    pub humidity: f64,
    pub wind_speed: f64,
    pub wind_degree: f64,
}

impl Default for WeatherInfo {
    fn default() -> Self {
        Self {
            city: "Kyiv".into(),
            weather: "Clear".into(),
            temp: 0.0,
            temp_feels_like: 0.0,
            pressure: 0.0,
            humidity: 0.0,
            wind_speed: 0.0,
            wind_degree: 0.0,
        }
    }
}

impl Display for WeatherInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "
City           : {}
Weather        : {}
Temperature    : {} °C
Feels Like     : {} °C
Pressure       : {} mBar
Humidity       : {} %
Wind Speed     : {:.0} m/s
Wind Direction : {} degree
            ",
            self.city,
            self.weather,
            self.temp,
            self.temp_feels_like,
            self.pressure,
            self.humidity,
            self.wind_speed,
            self.wind_degree
        )
    }
}
