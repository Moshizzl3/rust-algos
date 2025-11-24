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
    let mut bst = Bst::new();

    println!("Inserting 1, 2, 3, 4, 5, 6, 7...\n");

    for i in 1..=7 {
        avl.insert(i, i * 10);
    }
    println!("After inserting avl");
    println!("{}\n", avl);

    for i in 1..=7 {
        bst.insert(i, i * 10);
    }

    println!("After inserting bst");
    println!("{}\n", bst);
}
