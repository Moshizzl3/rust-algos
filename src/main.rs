mod algoritms;

use std::result;

use algoritms::recursive_stuff::{count_recursive, count_recursive_match, sum_recursive};
use algoritms::search::binary_search;
use algoritms::sorting::quick_sort::quick_sort;

use crate::algoritms::sorting::bubble_sort::bubble_sort;

fn main() {
    for size in [100, 500, 1000, 2000, 5000] {
        let list: Vec<i32> = (0..size).rev().collect();

        let start = std::time::Instant::now();
        let _ = quick_sort(&list);
        let quick_time = start.elapsed();

        let start = std::time::Instant::now();
        let _ = bubble_sort(&list);
        let bubble_time = start.elapsed();

        println!(
            "n={}: Quick={:?}, Bubble={:?}, Ratio={:.1}x",
            size,
            quick_time,
            bubble_time,
            bubble_time.as_micros() as f64 / quick_time.as_micros() as f64
        );
    }
}
