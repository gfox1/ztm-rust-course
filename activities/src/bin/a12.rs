// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics



// * Use an enum for the box color
enum BoxColor {
    Brown,
    White,
    Black
}

impl BoxColor {
    fn print(&self) {
        match self {
            BoxColor::Black => println!("black"),
            BoxColor::Brown => println!("brown"),
            BoxColor::White => println!("white")
        }
    }
}

struct Dimensions {
    depth: i32,
    width: i32,
    height: i32
}

impl Dimensions {
    fn print(&self) {
        println!("depth: {:?}", self.depth);
        println!("width: {:?}", self.width);
        println!("height {:?}", self.height);
    }
}
// * Use a struct to encapsulate the box characteristics
struct ShippingBox {
    dimensions: Dimensions,
    weight: f64,
    color: BoxColor
}


// * Implement functionality on the box struct to create a new box

impl ShippingBox {
    fn new_box (weight: f64, color: BoxColor, dimensions: Dimensions) -> Self {
        Self {
            weight, 
            color, 
            dimensions
        }
    }

// * Implement functionality on the box struct to print the characteristics

    fn print_box_specs (&self) {
       self.color.print();
       self.dimensions.print();
       println!("the box weight is: {:?}", self.weight);

    }
}



fn main() {
   let small_dimensions = Dimensions {
    depth: 5,
    width: 10,
    height: 15
   };

   let shipping_box = ShippingBox::new_box(10.0, BoxColor::Brown, small_dimensions);
   shipping_box.print_box_specs();
}
