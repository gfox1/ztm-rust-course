// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor



// * Use an enum to create different flavors of drinks
enum DrinkFlavours {
    Apple, 
    Orange,
    Cherry    
}

// * Use a struct to store drink flavor and fluid ounce information
struct Drink {
    drink_flavour: DrinkFlavours,
    fluid_ounce: f64
}
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor
fn drink_info (my_drink: Drink) {
    match my_drink.drink_flavour {
        DrinkFlavours::Apple => println!("flavour: apple"),
        DrinkFlavours::Orange => println!("flavour: orange"),
        DrinkFlavours::Cherry => println!("flavour: chery")
    }
    println!("oz: {:?}", my_drink.fluid_ounce);

}

fn main() {
    let my_drink = Drink {
        drink_flavour: DrinkFlavours::Cherry,
        fluid_ounce: 1.3423
    };

    let your_drink = Drink {
        drink_flavour: DrinkFlavours::Apple,
        fluid_ounce: 10.00
    };

    drink_info(my_drink);
    drink_info(your_drink);
    
}
