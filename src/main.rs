#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod models;
mod services;

use models::*;
use services::*;

use serde::{Deserialize, Serialize};



#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/flights")]
fn get_all_flights() -> String {
    let flights = build_flights();
    serde_json::to_string(&flights).unwrap()
}

fn main() {
    println!("Hello, world!");

    let mut flights = build_flights();

    let mut orders: Vec<Order> = Vec::new();

    println!("Flights: {:#?}", flights);

    rocket::ignite().mount("/", routes![index, get_all_flights]).launch();
}



fn build_flights() -> Vec<Flight> {
    let f1 = Flight {
        departure: Airports::DTW,
        arrival: Airports::JFK,
        price: 300.0,
        total_seats: 150,
        availabilities: Some(140),
    };
    
    let f2 = Flight {
        departure: Airports::DTW,
        arrival: Airports::JFK,
        price: 300.0,
        total_seats: 150,
        availabilities: None,
    };  

    let mut flights = Vec::new();
    flights.push(f1);
    flights.push(f2);
    flights
}





