// Topic: Looping using the loop statement
//
// Program requir&ements:
// * Display "1" through "4" in the terminal
//
// Notes:
// * Use a mutable integer variable
// * Use a loop statement
// * Print the variable within the loop statement
// * Use break to exit the loop

fn main() {

    let mut n = 1;

    loop {
        println!("{:?}", n);
        n = n + 1;
        if n == 5 {
            break;
        }
    }
    println!("Program Complete.")
        
    }
