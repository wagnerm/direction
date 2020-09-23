extern crate dotenv;

use dotenv::dotenv;
use std::env;

mod chat;
mod client;
mod point;

fn main() {
    dotenv().ok();

    let weather = client::WeatherClient::new("40.5853,-105.0844")
        .expect("Failed to initialize weather reading!");
    let f = weather.get_forecast().expect("Cound not get forecast!");

    let mut periods = vec![];
    for p in f.properties.periods {
        periods.push(format!("{} {} {}", p.name, p.windSpeed, p.windDirection));
    }

    let hook_url = get_chat_hook().unwrap();
    let c = chat::ChatClient::new(hook_url);
    c.post_message(periods.join("\n")).unwrap();
}

fn get_chat_hook() -> Result<String, std::env::VarError> {
    match env::var("CHAT_HOOK_URL") {
        Ok(hook_url) => Ok(hook_url),
        Err(_) => panic!("CHAT_HOOK_URL not present in environment!"),
    }
}
