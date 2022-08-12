// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

struct GroceryItem{
    quantity: i32,
    id_number: i32
}

fn display_qty (grocery: &GroceryItem){
    println!("the quantity is: {:?}", grocery.quantity)
}

fn display_id (grocery: &GroceryItem) {
    println!("the id is: {:?}", grocery.id_number)
}

fn main() {
    let milk = GroceryItem {
        quantity: 10,
        id_number: 1234
    };

    display_qty(&milk);
    display_id(&milk);


}
