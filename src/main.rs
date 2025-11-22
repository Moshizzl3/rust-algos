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

fn print_tree(node: &Rc<RefCell<TreeNode>>, prefix: String, is_left: bool) {
    let n = node.borrow();

    println!(
        "{}{}─ freq: {}, char: {:?}",
        prefix,
        if is_left { "├" } else { "└" },
        n.frequency,
        n.character
    );

    let new_prefix = format!("{}{}", prefix, if is_left { "│ " } else { "  " });

    if let Some(ref left) = n.left {
        print_tree(left, new_prefix.clone(), true);
    }
    if let Some(ref right) = n.right {
        print_tree(right, new_prefix, false);
    }
}
fn main() {
    let mut bst = Bst::new();

    bst.insert(5, "five");
    bst.insert(3, "three");
    bst.insert(7, "seven");
    bst.insert(1, "one");
    bst.insert(9, "nine");
    bst.insert(8, "eight");

    // Tree should look like:
    //       5
    //      / \
    //     3   7
    //    /     \
    //   1       9

    println!("Inserted!, {:?}", bst);
}
