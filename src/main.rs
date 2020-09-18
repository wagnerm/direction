mod api;
mod point;

fn main() {
    let weather = api::Weather::new().expect("Failed to initialize weather reading!");
    let p = point::Point::get(weather).expect("Failed to find point!");
    println!("Point is {:?}", p.properties.forecast);
}
