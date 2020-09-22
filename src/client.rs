extern crate reqwest;
extern crate serde;
extern crate serde_json;

use reqwest::blocking::Response;
use reqwest::header;
use std::error::Error;

use super::point;

pub struct WeatherClient<'c> {
    client: reqwest::blocking::Client,
    coordinate: &'c str,
}

impl<'c> WeatherClient<'c> {
    pub fn new(coordinate: &'c str) -> Result<WeatherClient, Box<dyn Error>> {
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

    pub fn get_forecast(&self) -> Result<point::Forecast, reqwest::Error> {
        let p = self.get_point().expect("Count not get point!");
        self.get_forecast_periods(&p)
    }

    fn get_point(&self) -> Result<point::Point, reqwest::Error> {
        let url = vec!["https://api.weather.gov/points/", self.coordinate].join("");
        let point: point::Point = self.get(&url)?.json()?;

        Ok(point)
    }

    fn get_forecast_periods(&self, p: &point::Point) -> Result<point::Forecast, reqwest::Error> {
        let url = String::from(p.properties.forecast.clone());
        let forecast: point::Forecast = self.get(&url)?.json()?;

        Ok(forecast)
    }

    fn get(&self, url: &String) -> Result<Response, reqwest::Error> {
        self.client.get(url).send()
    }
}
