mod client;
mod point;

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
    for p in f.properties.periods {
        println!("{} {} {}", p.name, p.windSpeed, p.windDirection);
    }
}
