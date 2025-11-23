mod algoritms;
mod data_structures;

use std::fs;
use std::time::{Duration, Instant};
use std::{cell::RefCell, collections::VecDeque, fs::read_dir, rc::Rc};

use crate::algoritms::compression::huffman::decode;
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
    let mut bst = Bst::new();

    // Build a balanced-ish tree
    bst.insert(50, "fifty");
    bst.insert(30, "thirty");
    bst.insert(70, "seventy");
    bst.insert(20, "twenty");
    bst.insert(40, "forty");
    bst.insert(60, "sixty");
    bst.insert(80, "eighty");
    bst.insert(10, "ten");
    bst.insert(25, "twenty-five");
    bst.insert(65, "sixty-five");
    bst.insert(90, "ninety");

    println!("My BST:\n{}", bst);

    // Delete some nodes to see it update
    bst.delete(&30);
    println!("\nAfter deleting 30:\n{}", bst);
}
