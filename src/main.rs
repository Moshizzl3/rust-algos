mod algoritms;
mod data_structures;

use std::{collections::VecDeque, fs::read_dir};

// use crate::data_structures::hash_tables::MoMap;
use crate::{
    algoritms::{compression::huffman::count_frequencies, search},
    data_structures::graphs::{MoGraph, Node},
};

fn main() {
    let text = "raspberry";
    let foo = count_frequencies(text);
    println!("{:?}", foo)
}