mod algoritms;
mod data_structures;

use std::{cell::RefCell, collections::VecDeque, fs::read_dir, rc::Rc};

// use crate::data_structures::hash_tables::MoMap;
use crate::{
    algoritms::{
        compression::huffman::{build_huffman_tree, count_frequencies},
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
    let text = "raspberry";
    let tree = build_huffman_tree(text).unwrap();
    print_tree(&tree, "".to_string(), false);
}
