use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Client {
    pub first_name: String,
    pub last_name: String,
}

#[derive(Debug)]
pub struct Voyager {
    pub first_name: String,
    pub last_name: String,
}

#[derive(Debug)]
pub struct Ticket {
    pub flight: Flight,
    pub voyageur: Voyager,
}

#[derive(Debug)]
pub struct Order {
    pub tickets: Vec<Ticket>,
    pub client: Client
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Flight {
    pub departure: Airports,
    pub arrival: Airports,
    pub price: f32,
    pub total_seats: i32,
    pub availabilities: Option<i32>,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum Airports {
    JFK,
    DTW,
    CDG,
}

#[derive(Debug)]
pub enum ErrorTypes {
    NoMoreSeats,
    TechnicalError,
}