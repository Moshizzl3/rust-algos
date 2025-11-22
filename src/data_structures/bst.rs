// Binary Search Tree (BST)

use core::borrow;
use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub struct BstNode<K, V> {
    pub key: K,
    pub value: V,
    left: Option<Rc<RefCell<BstNode<K, V>>>>,
    right: Option<Rc<RefCell<BstNode<K, V>>>>,
}

#[derive(Debug)]
pub struct Bst<K, V> {
    root: Option<Rc<RefCell<BstNode<K, V>>>>,
}

impl<K: Ord, V> Bst<K, V> {
    pub fn new() -> Self {
        Self { root: None }
    }
    pub fn insert(&mut self, key: K, value: V) {
        if self.root.is_none() {
            self.root = Some(Rc::new(RefCell::new(BstNode {
                key,
                value,
                left: None,
                right: None,
            })));
            return;
        }
        if let Some(ref node) = self.root {
            Self::insert_helper(node, key, value); //  associated function, cannot be called with self.foo
        }
    }

    fn insert_helper(node: &Rc<RefCell<BstNode<K, V>>>, key: K, value: V) {
        let borrowed = node.borrow();

        if key < borrowed.key {
            if let Some(ref left_node) = borrowed.left {
                Self::insert_helper(left_node, key, value);
                drop(borrowed);
            } else {
                drop(borrowed);
                node.borrow_mut().left = Some(Rc::new(RefCell::new(BstNode {
                    key,
                    value,
                    left: None,
                    right: None,
                })));
            }
        } else if key > borrowed.key {
            if let Some(ref right_node) = borrowed.right {
                Self::insert_helper(right_node, key, value);
                drop(borrowed);
            } else {
                drop(borrowed);
                node.borrow_mut().right = Some(Rc::new(RefCell::new(BstNode {
                    key,
                    value,
                    left: None,
                    right: None,
                })));
            }
        } else {
            drop(borrowed);
            node.borrow_mut().value = value
        }
    }
}

//TODO:
// pub fn search(&self, key: &K) -> Option<&V> {}
// pub fn delete(&mut self, key: &K) {}
