// AVL (Adelson-Velsky and Landis) binary search tree, self balancing tree,
// goal is to have O(log n) search time aka height.

use core::borrow;
use std::{cell::RefCell, rc::Rc};

type NodeRef<K, V> = Rc<RefCell<AVLNode<K, V>>>;

#[derive(Debug)]
pub struct AVLNode<K, V> {
    key: K,
    value: V,
    left: Option<NodeRef<K, V>>,
    right: Option<NodeRef<K, V>>,
    height: i32,
}
#[derive(Debug, Default)]
pub struct Avl<K, V> {
    root: Option<NodeRef<K, V>>,
}
