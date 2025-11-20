//simple Huffman algorithm for compressing text
use crate::data_structures::tree::TreeNode;
use core::borrow;
use std::cmp::Ordering;

use std::collections::BinaryHeap;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub fn count_frequencies(text: &str) -> HashMap<char, usize> {
    let mut freq_map = HashMap::new();

    text.chars()
        .for_each(|c| *freq_map.entry(c).or_insert(0) += 1);

    freq_map
}

pub fn build_huffman_tree(text: &str) -> Option<Rc<RefCell<TreeNode>>> {
    let freq_map = count_frequencies(text);
    if freq_map.is_empty() {
        return None;
    }

    //Create the leafs and add to tree

    let mut heap = BinaryHeap::new();
    for (ch, freq) in freq_map {
        let leaf = TreeNode::new_leaf(ch, freq);
        heap.push(HeapNode { node: leaf });
    }

    // Build tree

    while heap.len() > 1 {
        // pop smallest values of the heap - unwrap for now
        let left = heap.pop().unwrap().node;
        let right = heap.pop().unwrap().node;

        // create the parent
        let combined = left.borrow().frequency + right.borrow().frequency;
        let parent = TreeNode::new_internal(combined, left, right);

        //push back
        heap.push(HeapNode { node: parent });
    }

    //root node
    heap.pop().map(|heap_node| heap_node.node)
}

pub fn generate_codes(root: &Rc<RefCell<TreeNode>>) -> HashMap<char, String> {
    let mut codes = HashMap::new();

    generate_codes_helper(root, &mut String::new(), &mut codes);

    codes
}

pub fn generate_codes_helper(
    node: &Rc<RefCell<TreeNode>>,
    current_code: &mut String,
    codes: &mut HashMap<char, String>,
) {
    let borrowed_node = node.borrow();

    if let Some(char) = borrowed_node.character
        && borrowed_node.is_leaf()
    {
        codes.insert(char, current_code.to_owned());
        return;
    }

    let left_node = borrowed_node.left.clone();
    let right_node = borrowed_node.right.clone();
    drop(borrowed_node);

    if let Some(left) = left_node {
        current_code.push('0');
        generate_codes_helper(&left, current_code, codes);
        current_code.pop(); // backtrace
    }

    if let Some(right) = right_node {
        current_code.push('1');
        generate_codes_helper(&right, current_code, codes);
        current_code.pop(); //backtrace
    }
}

pub fn encode(text: &str, codes: &HashMap<char, String>) -> Result<String, String> {
    let mut result = String::new();

    for ch in text.chars() {
        match codes.get(&ch) {
            Some(code) => result.push_str(code),
            None => return Err(format!("Character '{}' not in codes", ch)),
        }
    }

    Ok(result)
}

pub fn decode(encoded: &str, tree: &Rc<RefCell<TreeNode>>) -> Result<String, String> {
    let mut result = String::new();
    let mut current_node = tree.clone();

    for bit in encoded.chars() {
        let borrowed = current_node.borrow();
        let left_node = borrowed.left.clone();
        let right_node = borrowed.right.clone();
        drop(borrowed);
        if bit == '0' {
            if let Some(ref left) = left_node {
                current_node = left.clone();
            } else {
                return Err(
                    "Invalid encoded string: tried to go left but no left child".to_string()
                );
            }
        } else if bit == '1' {
            if let Some(ref right) = right_node {
                current_node = right.clone();
            } else {
                return Err(
                    "Invalid encoded string: tried to go right but no right child".to_string(),
                );
            }
        } else {
            return Err(format!("Invalid bit: '{}' (expected '0' or '1')", bit));
        }
        let borrowed = current_node.borrow();

        if let Some(char) = borrowed.character
            && borrowed.is_leaf()
        {
            result.push(char);
            drop(borrowed);
            current_node = tree.clone()
        }
    }

    Ok(result)
}

pub struct HeapNode {
    node: Rc<RefCell<TreeNode>>,
}

impl Ord for HeapNode {
    // Reverse ordering: lower frequencies have higher priority
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .node
            .borrow()
            .frequency
            .cmp(&self.node.borrow().frequency)
    }
}

impl PartialOrd for HeapNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for HeapNode {
    fn eq(&self, other: &Self) -> bool {
        self.node.borrow().frequency == other.node.borrow().frequency
    }
}

impl Eq for HeapNode {}
