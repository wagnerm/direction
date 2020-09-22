extern crate reqwest;
extern crate serde;
extern crate serde_json;

use reqwest::blocking::Response;
use reqwest::header;
use std::error::Error;

pub struct Weather {
    client: reqwest::blocking::Client,
}

impl Weather {
    pub fn new() -> Result<Weather, Box<dyn Error>> {
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

        Ok(Weather { client: client })
    }

    pub fn get(&self, url: &String) -> Result<Response, reqwest::Error> {
        self.client.get(url).send()
    }
}
