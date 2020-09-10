mod api;

fn main() {
    let weather = api::Weather::new().expect("Failed to initialize weather reading!");

    weather.fetch_api().unwrap();
}
