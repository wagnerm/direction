extern crate reqwest;
extern crate serde;
extern crate serde_json;

use std::error::Error;
use std::fmt;

use serde::Serialize;

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
