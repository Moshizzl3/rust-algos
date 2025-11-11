mod algoritms;
mod data_structures;

use std::error::Error;
use std::result;

use algoritms::recursive_stuff::{count_recursive, count_recursive_match, sum_recursive};
use algoritms::search::binary_search;
use algoritms::sorting::quick_sort::quick_sort;

use crate::algoritms::sorting::bubble_sort::bubble_sort;
use crate::data_structures::hash_tables::{MoMap, MoMapError};

fn main() -> Result<(), MoMapError> {
    let mut my_map: MoMap<u32> = MoMap::new();

    my_map.bla();

    my_map.insert("hello".to_string(), 32);
    my_map.insert("hello1".to_string(), 32);
    my_map.insert("aaaa".to_string(), 42);
    my_map.insert("mohamad".to_string(), 11);
    my_map.bla();
    let test = my_map.get("mohamad").unwrap();

    println!("woow: {}", test);
    Ok(())
}
