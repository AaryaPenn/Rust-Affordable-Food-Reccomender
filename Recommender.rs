use reqwest;
use serde::{Deserialize, Serialize};
use std::env;
use std::io;

#[derive(Debug, Serialize, Deserialize)]
struct Location {
    lat: f64,
    lng: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct Geometry {
    location: Location,
}

#[derive(Debug, Serialize, Deserialize)]
struct OpeningHours {
    open_now: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct Place {
    name: String,
    vicinity: String,
    geometry: Geometry,
    opening_hours: Option<OpeningHours>,
}

#[derive(Debug, Serialize, Deserialize)]
struct PlacesResponse {
    results: Vec<Place>,
}

fn get_user_location() -> Result<Location, reqwest::Error> {
    // You may use an IP geolocation service to get the user's location based on their IP
    // For simplicity, we'll use a dummy location in this example
    Ok(Location { lat: 37.7749, lng: -122.4194 })
}

fn get_nearby_grocery_stores(api_key: &str, location: &Location, food_item: &str) -> Result<Vec<Place>, reqwest::Error> {
    let url = format!(
        "https://maps.googleapis.com/maps/api/place/nearbysearch/json?location={},{}&radius=5000&type=grocery_or_supermarket&keyword={}&key={}",
        location.lat, location.lng, food_item, api_key
    );

    let response: PlacesResponse = reqwest::blocking::get(&url)?.json()?;
    Ok(response.results)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let google_api_key = env::var("GOOGLE_API_KEY").expect("Please set the GOOGLE_API_KEY environment variable");
    let food_item: String = get_user_input("Enter the name of a food item: ")?;

    let user_location = get_user_location()?;
    let grocery_stores = get_nearby_grocery_stores(&google_api_key, &user_location, &food_item)?;

    if grocery_stores.is_empty() {
        println!("No nearby grocery stores found for {}.", food_item);
    } else {
        println!("Recommended grocery stores for {}: ", food_item);
        for store in grocery_stores {
            println!("{} - {}", store.name, store.vicinity);
        }
    }

    Ok(())
}

fn get_user_input(prompt: &str) -> Result<String, io::Error> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_owned())
}
