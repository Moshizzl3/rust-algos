mod algoritms;
mod data_structures;

use std::fs;
use std::time::{Duration, Instant};
use std::{cell::RefCell, collections::VecDeque, fs::read_dir, rc::Rc};

use crate::algoritms::compression::huffman::decode;
use crate::data_structures::avl_bst::Avl;
use crate::data_structures::b_tree::{BTree, BTreeNode};
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
    let mut tree = BTree::new(5);

    //just a test
    let root = Rc::new(RefCell::new(BTreeNode {
        keys: vec![10, 20, 30],
        values: vec!["ten", "twenty", "thirty"],
        children: None,
    }));

    tree.root = Some(root);

    println!("Search for 20: {:?}", tree.search(&20));
    println!("Search for 10: {:?}", tree.search(&10));
    println!("Search for 99: {:?}", tree.search(&99));
}
