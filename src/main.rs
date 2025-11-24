mod algoritms;
mod data_structures;

use std::fs;
use std::time::{Duration, Instant};
use std::{cell::RefCell, collections::VecDeque, fs::read_dir, rc::Rc};

use crate::algoritms::compression::huffman::decode;
use crate::data_structures::avl_bst::Avl;
use crate::data_structures::bst::Bst;
// use crate::data_structures::hash_tables::MoMap;
use crate::{
    algoritms::{
        compression::huffman::{build_huffman_tree, count_frequencies, encode, generate_codes},
        search,
    },
    data_structures::{
        graphs::{MoGraph, Node},
        tree::TreeNode,
    },
};

fn main() {
    let mut avl = Avl::new();

    for i in 1..=10 {
        avl.insert(i, i * 10);
    }

    println!("Tree:\n{}\n", avl);

    println!("Search for 5: {:?}", avl.search(&5));
    println!("Search for 10: {:?}", avl.search(&10));
    println!("Search for 99: {:?}", avl.search(&99));
}
