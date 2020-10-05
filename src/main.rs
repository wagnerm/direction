mod chat;
mod client;
mod point;

use std::env;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "direction")]
struct Opt {
    #[structopt(short, long)]
    coordinate: String,
}

fn main() {
    let opts = Opt::from_args();

    let weather =
        client::WeatherClient::new(opts.coordinate).expect("Failed to initialize weather reading!");
    let f = weather.get_forecast().expect("Cound not get forecast!");

    let hook_url = get_chat_hook().unwrap();
    let c = chat::ChatClient::new(hook_url);

    let message = c.format_message(f.properties.periods);
    c.post_message(message).unwrap();
}

fn get_chat_hook() -> Result<String, std::env::VarError> {
    match env::var("CHAT_HOOK_URL") {
        Ok(hook_url) => Ok(hook_url),
        Err(_) => panic!("CHAT_HOOK_URL not present in environment!"),
    }
}
