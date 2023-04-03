mod openweather;

use self::openweather::{CurrentWeather, OpenWeather};
use crate::error::{Error, Result};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub enum ProviderUserInfo {
    OpenWeather { api_key: String },
    WeatherApi,
}

impl ProviderUserInfo {
    pub fn build_provider(self) -> Box<dyn WeatherProvider> {
        match self {
            ProviderUserInfo::OpenWeather { api_key } => Box::new(OpenWeather::new(api_key)),
            ProviderUserInfo::WeatherApi => todo!(),
        }
    }

    pub fn from_file(file: &Path) -> Result<Self> {
        Ok(serde_json::from_reader(std::fs::File::open(file)?)?)
    }
}

#[async_trait]
pub trait WeatherProvider {
    async fn get_weather_city(&self, city: &str) -> Result<Weather>;
    async fn get_history_weather_city(&self, city: &str, date: DateTime<Utc>) -> Result<Weather>;
}

#[async_trait]
impl WeatherProvider for OpenWeather {
    async fn get_weather_city(&self, city: &str) -> Result<Weather> {
        let w = self.current_weather_city(city).await?;
        Ok(open_weather_extract_weather_data(w))
    }

    async fn get_history_weather_city(&self, city: &str, date: DateTime<Utc>) -> Result<Weather> {
        let weather = self.history_weather(city, date).await?;
        let ok_or = weather
            .list
            .into_iter()
            .next()
            .ok_or(Error::WeatherNoHistory)?;
        Ok(open_weather_extract_weather_data(ok_or))
    }
}

fn open_weather_extract_weather_data(w: CurrentWeather) -> Weather {
    Weather {
        cloudiness: w.clouds.all,
        rain_volume: w.rain.and_then(|x| x.n1h).unwrap_or(0.),
        temperature: w.main.temp,
        visibility: w.visibility,
        wind: Wind {
            speed: w.wind.speed,
            deg: w.wind.deg,
        },
        description: w
            .weather
            .first()
            .map(|x| x.description.clone())
            .unwrap_or_else(|| "".into()),
        location: format!("{}, {}", w.name, w.sys.country),
    }
}

#[derive(Debug)]
pub struct Weather {
    pub cloudiness: i64,
    pub description: String,
    pub temperature: f64,
    pub wind: Wind,
    pub rain_volume: f64,
    pub visibility: i64,
    pub location: String,
}

#[derive(Debug)]
pub struct Wind {
    /// Wind speed in m/s
    pub speed: f64,
    pub deg: i64,
}
