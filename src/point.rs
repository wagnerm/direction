use serde::{Deserialize, Serialize};

use super::api::Weather;

#[derive(Deserialize, Serialize)]
pub struct Point {
    id: String,
    pub properties: Property,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Property {
    pub forecast: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Forecast {
    pub properties: ForecastProperty,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ForecastProperty {
    pub periods: Vec<Period>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Period {
    pub number: usize,
    pub name: String,
    pub startTime: String,
    pub endTime: String,
    pub windSpeed: String,
    pub windDirection: String,
    pub detailedForecast: String,
}

impl Point {
    pub fn fetch(w: &Weather) -> Result<Point, reqwest::Error> {
        let url = String::from("https://api.weather.gov/points/40.5853,-105.0844");
        let point: Point = w.get(&url)?.json()?;

        Ok(point)
    }

    pub fn get_forecast(&self, w: &Weather) -> Result<Forecast, reqwest::Error> {
        let url = String::from(self.properties.forecast.clone());
        let forecast: Forecast = w.get(&url)?.json()?;

        Ok(forecast)
    }
}
