extern crate reqwest;
extern crate serde;
extern crate serde_json;

use reqwest::blocking::Response;
use reqwest::header;
use std::error::Error;

pub struct WeatherClient {
    client: reqwest::blocking::Client,
}

impl WeatherClient {
    pub fn new() -> Result<WeatherClient, Box<dyn Error>> {
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

        Ok(WeatherClient { client: client })
    }

    pub fn get(&self, url: &String) -> Result<Response, reqwest::Error> {
        self.client.get(url).send()
    }
}
