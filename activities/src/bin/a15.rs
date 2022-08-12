// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

enum Tickets {
    Backstage (String, f64),
    Vip (String, f64),
    Standard (f64)
}

fn main() {

    let tickets = vec![
        Tickets::Backstage("Bob".to_owned(), 100.00),
        Tickets::Vip("Alice".to_owned(), 75.00),
        Tickets::Standard(24.99)
    ];

    for n in tickets {
        match n {
            Tickets::Backstage(holder,price) => println!("{:?}, {:?}", holder,price),
            Tickets::Vip(holder,price) => println!("{:?}, {:?}", holder,price ),
            Tickets::Standard(price) => println!("{:?}",price )
        }
    };
}
