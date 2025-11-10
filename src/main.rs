mod algoritms;

use std::result;

use algoritms::recursive_stuff::{count_recursive, count_recursive_match, sum_recursive};
use algoritms::search::binary_search;

fn main() {
    println!("Hello, world!");

    let numbers: Vec<i32> = (0..=2).collect();

    let result = count_recursive(&numbers);

    println!("result: {}", result)
}
