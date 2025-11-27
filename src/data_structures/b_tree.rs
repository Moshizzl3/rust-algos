// B tree 🐝 (specialized m-way tree)

use core::borrow;
use std::{cell::RefCell, rc::Rc};

type NodeRef<K, V> = Rc<RefCell<BTreeNode<K, V>>>;

#[derive(Debug)]
pub struct BTreeNode<K, V> {
    pub keys: Vec<K>,
    pub values: Vec<V>,
    pub children: Option<Vec<NodeRef<K, V>>>,
}

#[derive(Debug)]
pub struct BTree<K, V> {
    pub root: Option<NodeRef<K, V>>,
    order: usize,
}

impl<K, V> BTreeNode<K, V> {
    fn is_leaf(&self) -> bool {
        self.children.is_none()
    }
}

impl<K: Ord + Clone, V: Clone> BTree<K, V> {
    pub fn new(order: usize) -> Self {
        assert!(order >= 3, "B-tree order must be at least 3");
        Self { root: None, order }
    }

    pub fn search(&self, key: &K) -> Option<V> {
        if let Some(ref root) = self.root {
            Self::search_helper(root, key)
        } else {
            None
        }
    }

 


    fn split_node(node: &NodeRef<K, V>) -> (K, V, NodeRef<K, V>) {
        let mut borrowed = node.borrow_mut();
        let median = borrowed.keys.len() / 2;

        let right_keys = borrowed.keys.split_off(median + 1);
        let right_values = borrowed.values.split_off(median + 1);

        let median_key = borrowed.keys.pop().unwrap();
        let median_value = borrowed.values.pop().unwrap();

        let right_children = borrowed
            .children
            .as_mut()
            .map(|children| children.split_off(median + 1));

        let right_node = Rc::new(RefCell::new(BTreeNode {
            keys: right_keys,
            values: right_values,
            children: right_children,
        }));

        (median_key, median_value, right_node)
    }

    fn search_helper(node: &NodeRef<K, V>, key: &K) -> Option<V> {
        let borrowed = node.borrow();

        match Self::find_key_index(&borrowed.keys, key) {
            Ok(idx) => Some(borrowed.values[idx].clone()),
            Err(idx) => {
                if borrowed.is_leaf() {
                    return None;
                }
                // get the child we need, then recurse
                let child = borrowed
                    .children
                    .as_ref()
                    .and_then(|x| x.get(idx))
                    .cloned()?;
                drop(borrowed);
                Self::search_helper(&child, key)
            }
        }
    }

    fn find_key_index(keys: &[K], key: &K) -> Result<usize, usize> {
        // we assume keys are sorted.
        debug_assert!(
            keys.is_sorted(),
            "Keys must be sorted, something is wrong with insert."
        );
        // important read docs to understand what it does, basically return index if found,
        // or the index where the key would go if not found
        keys.binary_search(key)
    }
}
