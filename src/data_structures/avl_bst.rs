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

impl<K: Ord + Clone, V: Clone> Avl<K, V> {
    pub fn new() -> Self {
        Self { root: None }
    }

    fn get_height(node: &Option<NodeRef<K, V>>) -> i32 {
        match node {
            Some(n) => n.borrow().height,
            None => 0,
        }
    }
    fn update_height(node: &NodeRef<K, V>) {
        let mut borrowed = node.borrow_mut();
        let left_height = Self::get_height(&borrowed.left);
        let right_height = Self::get_height(&borrowed.right);
        borrowed.height = 1 + std::cmp::max(left_height, right_height);
    }

    fn get_balance_factor(node: &NodeRef<K, V>) -> i32 {
        let borrowed = node.borrow();
        Self::get_height(&borrowed.left) - Self::get_height(&borrowed.right)
    }
    fn rotate_right(y: &NodeRef<K, V>) -> NodeRef<K, V> {
        let y_borrowed = y.borrow();
        let x = y_borrowed.left.clone().unwrap();
        drop(y_borrowed);

        let x_borrowed = x.borrow();
        let b = x_borrowed.right.clone();
        drop(x_borrowed);
        x.borrow_mut().right = Some(y.clone());
        y.borrow_mut().left = b;

        Self::update_height(y);
        Self::update_height(&x);

        x
    }
}
