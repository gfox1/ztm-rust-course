// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

// * Use an enum with color names as variants
enum Colours {
    Blue,
    Pink,
    Purple,
    Green,
    Black
}
// * Use a function to print the color name
// * The function must use the enum as a parameter
fn print_colour (colour : Colours) {
// * Use a match expression to determine which color
//   name to print
    match colour {
        Colours::Blue => println!("blue"),
        Colours::Pink => println!("pink"),
        Colours::Purple => println!("purple"),
        Colours::Green => println!("green"),
        _ => println!("Dont care about this colour")
    }
}

fn main() {
    
    let colour = Colours::Black;
    print_colour(colour);
    
}
