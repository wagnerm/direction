mod api;
mod point;

fn main() {
    let weather = api::Weather::new().expect("Failed to initialize weather reading!");
    let p = point::Point::fetch(&weather).expect("Failed to find point!");
    let f = p.get_forecast(&weather).expect("Cound not get forecast!");
    for p in f.properties.periods {
        println!("{} {} {}", p.name, p.windSpeed, p.windDirection);
    }
}
