// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

struct Person {
    age: i32,
    name: String,
    fav_color: String
}

fn print_name (name: &str) {
    println!("name: {:?}", name);
}

fn print_fav_color (fav_color: &str) {
    println!("Fav Color: {:?}", fav_color);
}

fn main() {

    let people_vec = vec![
        Person {
            age: 5,
            name: "alice".to_owned(),
            fav_color: "pink".to_owned()
        },

        Person {
            age: 10,
            name: String::from("Bob"),
            fav_color: String::from("blue")
        },

        Person {
            age: 20,
            name: String::from("jim"),
            fav_color: String::from("green")
        }
    ];

    for n in &people_vec {
        if n.age <= 10 {
            // println!("Name: {:?}, Favourite Color: {:?}", n.name, n.fav_color);
            print_name(&n.name);
            print_fav_color(&n.fav_color);
        } else {
            return
        }
    };

}
