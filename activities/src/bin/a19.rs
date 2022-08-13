// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock

use std::collections::HashMap;


fn main() {

    let mut furniture_in_stock = HashMap::new();
    furniture_in_stock.insert("Chair", 5);
    furniture_in_stock.insert("Bed", 3);
    furniture_in_stock.insert("Table", 2);
    furniture_in_stock.insert("Couch", 0);

    let mut total_stock = 0;
    for (furniture, stock) in furniture_in_stock.iter() {
        total_stock = total_stock + stock;
        if stock == &0 {
            println!("Furniture: {:?} - Out of stock", furniture)
        } else {        
        println!("Furniture: {:?} - Stock: {:?}", furniture, stock)
        }
    }   

    println!("total stock = {:?}", total_stock);
}
