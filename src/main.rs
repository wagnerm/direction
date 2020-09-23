extern crate dotenv;

use dotenv::dotenv;

mod chat;
mod client;
mod point;

fn main() {
    dotenv().ok();

    let weather = client::WeatherClient::new("40.5853,-105.0844")
        .expect("Failed to initialize weather reading!");
    let f = weather.get_forecast().expect("Cound not get forecast!");
    for p in f.properties.periods {
        println!("{} {} {}", p.name, p.windSpeed, p.windDirection);
    }
}
