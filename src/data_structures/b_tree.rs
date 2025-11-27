// B tree 🐝 (specialized m-way tree)

use std::{cell::RefCell, rc::Rc};

type NodeRef<K, V> = Rc<RefCell<BTreeNode<K, V>>>;

#[derive(Debug)]
pub struct BTreeNode<K, V> {
    keys: Vec<K>,
    values: Vec<V>,
    children: Option<Vec<NodeRef<K, V>>>,
}

#[derive(Debug)]
pub struct BTree<K, V> {
    root: Option<NodeRef<K, V>>,
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

    fn search_helper(node: &NodeRef<K, V>, key: &K) {}

    fn find_key_index(keys: &[K], key: &K) -> Result<usize, usize> {
        // we assume keys are sorted.
        debug_assert!(
            keys.is_sorted(),
            "Keys must be sorted, something is wrong with insert."
        );
        keys.binary_search(key)
    }
}
