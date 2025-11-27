use std::io;
use serde::Deserialize
use colored::*;

//Struct to desieralize the JSON response from openWeatherMap API
#[derive(Deserialize, Debug)]
struct WeatherResponse {
    weather: Vec<Weather>,
    main: Main,
    wind: Wind,
    name: String,
}

//Struct to desieralize the weather part of the JSON response
#[derive(Deserialize, Debug)]
struct Weather {
    description: String,
}

//Struct to desieralize the main part of the JSON response
#[derive(Deserialize, Debug)]
struct Main {
    temp: f64,
    humidity: f64,
    pressure: f64,
}

//Struct to desieralize the wind part of the JSON response
#[derive(Deserialize, Debug)]
struct Wind {
    speed: f64,
    //deg: f64,
}

fn main() {
    println!("Hello, world!");
}
