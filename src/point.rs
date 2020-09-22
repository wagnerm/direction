use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Point {
    pub id: String,
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
