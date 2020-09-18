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

impl Point {
    pub fn get(w: Weather) -> Result<Point, reqwest::Error> {
        let url = String::from("https://api.weather.gov/points/40.5853,-105.0844");
        let json: Point = w.get(&url)?.json()?;

        Ok(json)
    }
}
