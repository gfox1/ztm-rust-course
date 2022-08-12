// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuple
// * Destructure the return value into two variables
// * Use an if..else if..else block to determine what to print


// * Use a function that returns a tuple
fn coordinate (a: i32, b: i32) -> (i32, i32){
    (a, b)
}

fn main() {
// * Destructure the return value into two variables
    let (x,y) = coordinate(2,6);

// * Use an if..else if..else block to determine what to print

    if y > 5 {
        println!("greater than 5")
    } else if  y < 5{
        println!("less than 5")
    } else {
        println!("y equals 5")
    }

}
