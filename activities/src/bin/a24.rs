// Topic: Iterator
//
// Requirements:
// * Triple the value of each item in a vector.
// * Filter the data to only include values > 10.
// * Print out each element using a for loop.
//
// Notes:
// * Use an iterator chain to accomplish the task.

fn main() {
    let data = vec![1, 2, 3, 4, 5];

    let data_triple: Vec<_> = data
        .iter()
        .map(|num| num * 3)
        .collect();

    let over_ten: Vec<_> = data_triple
        .iter()
        .filter(|num| num > &&10)
        .collect();

    for n in over_ten {
        println!("{:?}", n)
    }
}
