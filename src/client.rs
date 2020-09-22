extern crate reqwest;
extern crate serde;
extern crate serde_json;

use reqwest::blocking::Response;
use reqwest::header;
use std::error::Error;

use super::point;

pub struct WeatherClient {
    client: reqwest::blocking::Client,
    coordinate: String,
}

impl WeatherClient {
    pub fn new(coordinate: String) -> Result<WeatherClient, Box<dyn Error>> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            header::USER_AGENT,
            "Direction / https://github.com/wagnerm/direction"
                .parse()
                .unwrap(),
        );
        let client = reqwest::blocking::Client::builder()
            .default_headers(headers)
            .build()?;

        Ok(WeatherClient {
            client: client,
            coordinate: coordinate,
        })
    }

    pub fn get(&self, url: &String) -> Result<Response, reqwest::Error> {
        self.client.get(url).send()
    }

    pub fn get_point(&self) -> Result<point::Point, reqwest::Error> {
        let url = String::from("https://api.weather.gov/points/***REMOVED***");
        let point: point::Point = self.get(&url)?.json()?;

        Ok(point)
    }

    pub fn get_forecast(&self, p: &point::Point) -> Result<point::Forecast, reqwest::Error> {
        let url = String::from(p.properties.forecast.clone());
        let forecast: point::Forecast = self.get(&url)?.json()?;

        Ok(forecast)
    }
}
