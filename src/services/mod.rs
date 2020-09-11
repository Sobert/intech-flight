use super::models::*;


fn verify_order(flight: Flight) -> Result<Flight, ErrorTypes> {
    match flight.availabilities {
        Some(v) => {
            println!("places disponibles: {:#?}", v);
            Ok(flight)
        },
        None => { 
            println!("plus de places disponibles");
            Err(ErrorTypes::NoMoreSeats) 
        },
    }   
}

fn place_order(mut orders: Vec<Order>, 
    mut flights: &Vec<Flight>,
    order: Order) -> Result<Vec<Order>, ErrorTypes> {
        for ticket in &order.tickets {
            match verify_order(ticket.flight) {
                Ok(v) => {
                    println!("ticket ok");
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }

        orders.push(order);
        Ok(orders)
}