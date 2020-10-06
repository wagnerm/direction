extern crate reqwest;
extern crate serde;
extern crate serde_json;

use serde::Serialize;
use std::error::Error;
use std::fmt;

use super::point::Period;

#[derive(Debug)]
struct ChatClientError(String);

impl Error for ChatClientError {}

impl fmt::Display for ChatClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error posting to chat: {}", self.0)
    }
}

#[derive(Serialize)]
struct Message {
    text: String,
}

pub struct ChatClient {
    hook_url: String,
}

impl ChatClient {
    pub fn new(hook_url: String) -> ChatClient {
        ChatClient { hook_url: hook_url }
    }

    pub fn format_message(&self, periods: Vec<Period>) -> String {
        let mut lines: Vec<String> = vec![];
        lines.push(String::from("```"));

        let max_name_length = {
            let mut max = 10;
            for p in periods.iter() {
                if p.name.len() > max {
                    max = p.name.len();
                }
            }

            max
        };

        let max_speed_length = {
            let mut max = 6;
            for p in periods.iter() {
                if p.windSpeed.len() > max {
                    max = p.windSpeed.len();
                }
            }

            max
        };

        let max_dir_length = {
            let mut max = 10;
            for p in periods.iter() {
                if p.windDirection.len() > max {
                    max = p.windDirection.len();
                }
            }

            max
        };

        lines.push(format!(
            "{:width_name$} {:width_speed$} {:width_dir$}",
            "Timeframe:",
            "Speed:",
            "Direction:",
            width_name = max_name_length,
            width_speed = max_speed_length,
            width_dir = max_dir_length
        ));

        for p in periods.iter() {
            lines.push(format!(
                "{:width_name$} {:width_speed$} {:width_dir$}",
                p.name,
                p.windSpeed,
                p.windDirection,
                width_name = max_name_length,
                width_speed = max_speed_length,
                width_dir = max_dir_length
            ))
        }

        lines.push(String::from("```"));
        lines.join("\n")
    }

    pub fn post_message(&self, message: String) -> Result<(), Box<dyn Error>> {
        let new_message = Message { text: message };
        let response = reqwest::blocking::Client::new()
            .post(&self.hook_url)
            .json(&new_message)
            .send()?
            .text()?;

        if response != "ok" {
            Err(Box::new(ChatClientError(response)))
        } else {
            Ok(())
        }
    }
}
