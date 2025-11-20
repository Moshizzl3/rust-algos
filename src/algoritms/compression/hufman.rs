//simple Hufman algorithm for compressing text
use crate::data_structures::tree::TreeNode;
use std::cmp::Ordering;

use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub fn count_frequencies(text: &str) -> HashMap<char, i32> {
    let mut freq_map = HashMap::new();

    text.chars()
        .for_each(|c| *freq_map.entry(c).or_insert(0) += 1);

    freq_map
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
