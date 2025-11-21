mod algoritms;
mod data_structures;

use std::fs;
use std::time::{Duration, Instant};
use std::{cell::RefCell, collections::VecDeque, fs::read_dir, rc::Rc};

use crate::algoritms::compression::huffman::decode;
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
    let text = fs::read_to_string("./data/moby_dick.txt").expect("Failed to read file");
    let text_encoded = fs::read_to_string("./data/encoded.txt").expect("Failed to read file");

    println!("File size: {} characters", text.len());
    let start = Instant::now();
    // Build tree
    println!("Building Huffman tree...");
    let tree = build_huffman_tree(&text).unwrap();

    println!("Generating codes...");
    let codes = generate_codes(&tree);

    println!("Encoding...");
    let encoded = encode(&text, &codes).unwrap();

    // Stats
    let duration = start.elapsed();
    fs::write("./data/encoded.txt", &encoded).unwrap();
    println!("Time elapsed: {:?} ms", duration.as_millis());
    let original_bits = text.len() * 8;
    let compressed_bits = encoded.len();
    let ratio = (1.0 - (compressed_bits as f64 / original_bits as f64)) * 100.0;

    println!("\nResults:");
    println!(
        "Original: {} bits ({} KB)",
        original_bits,
        text.len() / 1024
    );
    println!(
        "Compressed: {} bits ({} KB)",
        compressed_bits,
        compressed_bits / 8 / 1024
    );
    println!("Compression: {:.2}%", ratio);

    let decoded = decode(&text_encoded, &tree).unwrap();
    fs::write("./data/moby_dick2.txt", &decoded).unwrap();
    if text == decoded {
        println!("SUCCESS! Decoded text matches original!");
        println!(
            "Compressed {} chars to {} bits and back!",
            text.len(),
            encoded.len()
        );
    } else {
        println!("ERROR: Decoded text doesn't match!");
    }
}
