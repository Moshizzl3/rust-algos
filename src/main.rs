mod algoritms;

use std::result;

use algoritms::binary_search::binary_search;

fn main() {
    println!("Hello, world!");

    let numbers: Vec<i32> = (0..=1000).collect();

    let result = binary_search(&numbers, 1000);

    if let Some(value) = result {
        println!("result {}", value)
    } else {
        println!("Value not found")
    }
}
