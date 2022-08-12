// Topic: Option
//
// Requirements:
// * Print out the details& of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>


struct LockerAsignment {
    locker_number : Option<i32>,
    student_name : String
}


fn main() {

    let student_1 = LockerAsignment {
        student_name: "Bob".to_owned(),
        locker_number: Some(3)
    };

    println!("stundet: {:?}", student_1.student_name);
    match student_1.locker_number {
        Some(num) => println!("locker number: {:?}",num),
        None => println!("No locker assigned.")
    };
}
