mod algoritms;

use std::result;

use algoritms::recursive_stuff::{count_recursive, count_recursive_match, sum_recursive};
use algoritms::search::binary_search;
use algoritms::sorting::quick_sort::quick_sort;

fn main() {
    println!("Hello, world!");

    let numbers: Vec<i32> = vec![2, 4, 3, 5, 1, 8, 7, 9, -1, 0];

    let result = quick_sort(&numbers);

    println!("result: {:?}", result);
}
