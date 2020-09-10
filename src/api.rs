extern crate reqwest;
extern crate serde;
extern crate serde_json;

use serde::Deserialize;

use reqwest::header;
use std::error::Error;

#[derive(Deserialize)]
pub struct Status {
  status: String,
}

pub struct Weather {
    client: reqwest::blocking::Client,
}

pub trait Requester {
    fn get(&self, url: &String) -> Result<Status, reqwest::Error>;
}

impl Requester for Weather {
    fn get(&self, url: &String) -> Result<Status, reqwest::Error> {
        let json: Status = self.client.get(url).send()?.json()?;
        Ok(json)
    }
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

    pub fn fetch_api(&self) -> Result<(), Box<dyn Error>> {
        let query = String::from("https://api.weather.gov");
        let json = &self.get(&query)?;
        println!("{}", json.status);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt;
}
