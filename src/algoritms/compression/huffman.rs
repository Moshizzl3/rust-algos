//simple Huffman algorithm for compressing text
use crate::data_structures::tree::TreeNode;
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
