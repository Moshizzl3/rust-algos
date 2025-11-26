// AVL (Adelson-Velsky and Landis) binary search tree, self balancing tree,
// goal is to have O(log n) search time aka height.

use core::{borrow, fmt};
use std::{cell::RefCell, fmt::Display, rc::Rc};

use crate::data_structures::graphs::Node;

type NodeRef<K, V> = Rc<RefCell<AVLNode<K, V>>>;

#[derive(Debug)]
pub struct AVLNode<K, V> {
    pub key: K,
    pub value: V,
    pub left: Option<NodeRef<K, V>>,
    pub right: Option<NodeRef<K, V>>,
    pub height: i32,
}
#[derive(Debug, Default)]
pub struct Avl<K, V> {
    root: Option<NodeRef<K, V>>,
}

impl<K: Ord + Clone, V: Clone> Avl<K, V> {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn search(&self, key: &K) -> Option<V> {
        if let Some(ref node) = self.root {
            Self::search_helper(node, key)
        } else {
            None
        }
    }

    fn search_helper(node: &NodeRef<K, V>, key: &K) -> Option<V> {
        let borrowed = node.borrow();

        if *key == borrowed.key {
            return Some(borrowed.value.clone());
        }

        if *key < borrowed.key {
            if let Some(ref left_node) = borrowed.left.clone() {
                drop(borrowed);
                Self::search_helper(left_node, key)
            } else {
                None
            }
        } else if let Some(ref right_node) = borrowed.right.clone() {
            drop(borrowed);
            Self::search_helper(right_node, key)
        } else {
            None
        }
    }

    pub fn delete(&mut self, key: &K) {
        if let Some(ref root) = self.root {
            self.root = Self::delete_helper(root, key)
        }
    }

    fn delete_helper(node: &NodeRef<K, V>, key: &K) -> Option<NodeRef<K, V>> {
        let mut borrowed = node.borrow_mut();
        if *key < borrowed.key {
            if let Some(left) = borrowed.left.take() {
                borrowed.left = Self::delete_helper(&left, key);
            }
        } else if *key > borrowed.key {
            if let Some(right) = borrowed.right.take() {
                borrowed.right = Self::delete_helper(&right, key);
            }
        } else {
            // If leaf (no children)
            if borrowed.left.is_none() && borrowed.right.is_none() {
                return None;
            }
            // Parent node has only one child
            if borrowed.left.is_none() {
                return borrowed.right.take();
            }
            if borrowed.right.is_none() {
                return borrowed.left.take();
            }
            // Parent node has two children
            // If this panic, something is totally wrong, since have checked for none above
            let right = borrowed
                .right
                .take()
                .expect("Two children case, right child should exist, should not be None.");

            let (successor_key, successor_value) = Self::find_min(&right);
            borrowed.key = successor_key;
            borrowed.value = successor_value;
            borrowed.right = Self::delete_helper(&right, &borrowed.key); //use successor key here
        }

        drop(borrowed);
        Self::update_height(node);
        let balance_factor = Self::get_balance_factor(node);

        if balance_factor > 1 {
            let left_node = node
                .borrow()
                .left
                .clone()
                .expect("Left child must exist, since balance_factor > 1");
            let left_balance = Self::get_balance_factor(&left_node);

            if left_balance >= 0 {
                return Some(Self::rotate_right(node));
            }
            return Some(Self::rotate_left_right(node));
        }

        if balance_factor < -1 {
            let right_node = node
                .borrow()
                .right
                .clone()
                .expect("Right child must exist since balance_factor < -1");
            let right_balance = Self::get_balance_factor(&right_node);
            if right_balance <= 0 {
                return Some(Self::rotate_left(node));
            }
            return Some(Self::rotate_right_left(node));
        }
        Some(node.clone())
    }

    fn find_min(node: &Rc<RefCell<AVLNode<K, V>>>) -> (K, V) {
        let borrowed = node.borrow();
        if let Some(ref left) = borrowed.left {
            return Self::find_min(left);
        }
        (borrowed.key.clone(), borrowed.value.clone())
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    pub fn contains(&self, key: &K) -> bool {
        self.search(key).is_some()
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
    fn rotate_left(y: &NodeRef<K, V>) -> NodeRef<K, V> {
        let y_borrowed = y.borrow();
        let x = y_borrowed.right.clone().unwrap();
        drop(y_borrowed);

        let x_borrowed = x.borrow();
        let b = x_borrowed.left.clone();
        drop(x_borrowed);
        x.borrow_mut().left = Some(y.clone());
        y.borrow_mut().right = b;

        Self::update_height(y);
        Self::update_height(&x);

        x
    }

    fn rotate_left_right(z: &NodeRef<K, V>) -> NodeRef<K, V> {
        let z_borrow = z.borrow();
        let x = z_borrow.left.clone().unwrap();
        drop(z_borrow);
        let rotated_left = Self::rotate_left(&x);
        z.borrow_mut().left = Some(rotated_left);

        Self::rotate_right(z)
    }
    fn rotate_right_left(z: &NodeRef<K, V>) -> NodeRef<K, V> {
        let z_borrow = z.borrow();
        let x = z_borrow.right.clone().unwrap();
        drop(z_borrow);
        let rotated_right = Self::rotate_right(&x);
        z.borrow_mut().right = Some(rotated_right);

        Self::rotate_left(z)
    }

    pub fn insert(&mut self, key: K, value: V) {
        if let Some(ref root) = self.root {
            self.root = Some(Self::insert_helper(root, key, value))
        } else {
            self.root = Some(Rc::new(RefCell::new(AVLNode {
                key,
                value,
                left: None,
                right: None,
                height: 1,
            })))
        }
    }

    fn insert_helper(node: &NodeRef<K, V>, key: K, value: V) -> NodeRef<K, V> {
        let borrowed = node.borrow();
        if key < borrowed.key {
            if let Some(ref left_node) = borrowed.left {
                let left_clone = left_node.clone();
                drop(borrowed);
                node.borrow_mut().left = Some(Self::insert_helper(&left_clone, key, value));
            } else {
                drop(borrowed);
                node.borrow_mut().left = Some(Rc::new(RefCell::new(AVLNode {
                    key,
                    value,
                    left: None,
                    right: None,
                    height: 1,
                })))
            }
        } else if key > borrowed.key {
            if let Some(ref right_node) = borrowed.right {
                let right_clone = right_node.clone();
                drop(borrowed);
                node.borrow_mut().right = Some(Self::insert_helper(&right_clone, key, value))
            } else {
                drop(borrowed);
                node.borrow_mut().right = Some(Rc::new(RefCell::new(AVLNode {
                    key,
                    value,
                    left: None,
                    right: None,
                    height: 1,
                })));
            }
        } else {
            drop(borrowed);
            node.borrow_mut().value = value
        }
        Self::update_height(node);
        let balance_factor = Self::get_balance_factor(node);

        if balance_factor > 1 {
            let left_node = node
                .borrow()
                .left
                .clone()
                .expect("Left child must exist, since balance_factor > 1");
            let left_balance = Self::get_balance_factor(&left_node);

            if left_balance >= 0 {
                return Self::rotate_right(node);
            }
            return Self::rotate_left_right(node);
        }

        if balance_factor < -1 {
            let right_node = node
                .borrow()
                .right
                .clone()
                .expect("Right child must exist since balance_factor < -1");
            let right_balance = Self::get_balance_factor(&right_node);
            if right_balance <= 0 {
                return Self::rotate_left(node);
            }
            return Self::rotate_right_left(node);
        }
        node.clone()
    }
}

// custom display implementation
impl<K: Display + Ord, V: Display> Display for Avl<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref root) = self.root {
            Self::display_helper(root, f, "", "", true)
        } else {
            write!(f, "Empty tree")
        }
    }
}

impl<K: Ord + Display, V: Display> Avl<K, V> {
    fn display_helper(
        node: &Rc<RefCell<AVLNode<K, V>>>,
        f: &mut fmt::Formatter<'_>,
        prefix: &str,
        child_prefix: &str,
        is_root: bool,
    ) -> fmt::Result {
        let borrowed = node.borrow();
        let left_borrowed = borrowed.left.clone();
        let right_borrowed = borrowed.right.clone();

        if is_root {
            writeln!(f, "{}", borrowed.key)?;
        } else {
            writeln!(f, "{}{}", prefix, borrowed.key)?;
        }
        drop(borrowed);

        if let Some(left) = left_borrowed {
            let new_prefix = format!("{}├── ", child_prefix);
            let new_child_prefix = if right_borrowed.is_some() {
                format!("{}│   ", child_prefix)
            } else {
                format!("{}    ", child_prefix)
            };
            Self::display_helper(&left, f, &new_prefix, &new_child_prefix, false)?;
        }
        if let Some(ref right) = right_borrowed {
            let new_prefix = format!("{}└── ", child_prefix);
            let new_child_prefix = format!("{}    ", child_prefix);

            Self::display_helper(right, f, &new_prefix, &new_child_prefix, false)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod avl_tests {
    use super::*;

    #[test]
    fn test_new_avl_is_empty() {
        let avl: Avl<i32, String> = Avl::new();
        assert!(avl.root.is_none());
    }

    #[test]
    fn test_insert_single() {
        let mut avl = Avl::new();
        avl.insert(5, "five");

        assert!(avl.root.is_some());
        assert_eq!(avl.search(&5), Some("five"));
    }

    #[test]
    fn test_insert_and_search() {
        let mut avl = Avl::new();
        avl.insert(5, "five");
        avl.insert(3, "three");
        avl.insert(7, "seven");

        assert_eq!(avl.search(&5), Some("five"));
        assert_eq!(avl.search(&3), Some("three"));
        assert_eq!(avl.search(&7), Some("seven"));
        assert_eq!(avl.search(&10), None);
    }

    #[test]
    fn test_insert_updates_existing() {
        let mut avl = Avl::new();
        avl.insert(5, "five");
        avl.insert(5, "FIVE");

        assert_eq!(avl.search(&5), Some("FIVE"));
    }

    #[test]
    fn test_right_rotation_ll_case() {
        let mut avl = Avl::new();
        avl.insert(3, "three");
        avl.insert(2, "two");
        avl.insert(1, "one"); // Triggers LL rotation

        // Root should be 2 after rotation
        let root = avl.root.as_ref().unwrap();
        assert_eq!(root.borrow().key, 2);

        // All values should still be searchable
        assert_eq!(avl.search(&1), Some("one"));
        assert_eq!(avl.search(&2), Some("two"));
        assert_eq!(avl.search(&3), Some("three"));
    }

    #[test]
    fn test_left_rotation_rr_case() {
        let mut avl = Avl::new();
        avl.insert(1, "one");
        avl.insert(2, "two");
        avl.insert(3, "three"); // Triggers RR rotation

        // Root should be 2 after rotation
        let root = avl.root.as_ref().unwrap();
        assert_eq!(root.borrow().key, 2);

        assert_eq!(avl.search(&1), Some("one"));
        assert_eq!(avl.search(&2), Some("two"));
        assert_eq!(avl.search(&3), Some("three"));
    }

    #[test]
    fn test_left_right_rotation_lr_case() {
        let mut avl = Avl::new();
        avl.insert(3, "three");
        avl.insert(1, "one");
        avl.insert(2, "two"); // Triggers LR rotation

        // Root should be 2 after rotation
        let root = avl.root.as_ref().unwrap();
        assert_eq!(root.borrow().key, 2);

        assert_eq!(avl.search(&1), Some("one"));
        assert_eq!(avl.search(&2), Some("two"));
        assert_eq!(avl.search(&3), Some("three"));
    }

    #[test]
    fn test_right_left_rotation_rl_case() {
        let mut avl = Avl::new();
        avl.insert(1, "one");
        avl.insert(3, "three");
        avl.insert(2, "two"); // Triggers RL rotation

        // Root should be 2 after rotation
        let root = avl.root.as_ref().unwrap();
        assert_eq!(root.borrow().key, 2);

        assert_eq!(avl.search(&1), Some("one"));
        assert_eq!(avl.search(&2), Some("two"));
        assert_eq!(avl.search(&3), Some("three"));
    }

    #[test]
    fn test_sorted_insert_stays_balanced() {
        let mut avl = Avl::new();

        // Insert 1-10 in order (worst case for BST)
        for i in 1..=10 {
            avl.insert(i, i * 10);
        }

        // Check all values are findable
        for i in 1..=10 {
            assert_eq!(avl.search(&i), Some(i * 10));
        }

        // Height should be log(n), not n
        // For 10 nodes, height should be ~4, not 10
        let root = avl.root.as_ref().unwrap();
        assert!(root.borrow().height <= 5); // Allow some slack
    }

    #[test]
    fn test_reverse_sorted_insert_stays_balanced() {
        let mut avl = Avl::new();

        // Insert 10-1 in reverse order
        for i in (1..=10).rev() {
            avl.insert(i, i * 10);
        }

        for i in 1..=10 {
            assert_eq!(avl.search(&i), Some(i * 10));
        }

        let root = avl.root.as_ref().unwrap();
        assert!(root.borrow().height <= 5);
    }

    #[test]
    fn test_delete_leaf() {
        let mut avl = Avl::new();
        avl.insert(5, "five");
        avl.insert(3, "three");
        avl.insert(7, "seven");

        avl.delete(&3);

        assert_eq!(avl.search(&3), None);
        assert_eq!(avl.search(&5), Some("five"));
        assert_eq!(avl.search(&7), Some("seven"));
    }

    #[test]
    fn test_delete_node_with_one_child() {
        let mut avl = Avl::new();
        avl.insert(5, "five");
        avl.insert(3, "three");
        avl.insert(7, "seven");
        avl.insert(6, "six");

        avl.delete(&7); // 7 has only left child (6)

        assert_eq!(avl.search(&7), None);
        assert_eq!(avl.search(&6), Some("six"));
    }

    #[test]
    fn test_delete_node_with_two_children() {
        let mut avl = Avl::new();
        for i in 1..=7 {
            avl.insert(i, i * 10);
        }

        avl.delete(&4); // Node with two children

        assert_eq!(avl.search(&4), None);
        // All other nodes should still exist
        for i in [1, 2, 3, 5, 6, 7] {
            assert_eq!(avl.search(&i), Some(i * 10));
        }
    }

    #[test]
    fn test_delete_root() {
        let mut avl = Avl::new();
        avl.insert(5, "five");
        avl.insert(3, "three");
        avl.insert(7, "seven");

        avl.delete(&5);

        assert_eq!(avl.search(&5), None);
        assert_eq!(avl.search(&3), Some("three"));
        assert_eq!(avl.search(&7), Some("seven"));
    }

    #[test]
    fn test_delete_all_nodes() {
        let mut avl = Avl::new();
        for i in 1..=5 {
            avl.insert(i, i * 10);
        }

        for i in 1..=5 {
            avl.delete(&i);
        }

        for i in 1..=5 {
            assert_eq!(avl.search(&i), None);
        }
    }

    #[test]
    fn test_delete_nonexistent() {
        let mut avl = Avl::new();
        avl.insert(5, "five");

        avl.delete(&10); // Doesn't exist

        // Tree should be unchanged
        assert_eq!(avl.search(&5), Some("five"));
    }

    #[test]
    fn test_delete_maintains_balance() {
        let mut avl = Avl::new();

        for i in 1..=10 {
            avl.insert(i, i * 10);
        }

        // Delete several nodes
        avl.delete(&1);
        avl.delete(&2);
        avl.delete(&3);

        // Tree should still be balanced
        let root = avl.root.as_ref().unwrap();
        let balance = get_balance_from_outside(&root);
        assert!(balance >= -1 && balance <= 1);
    }

    #[test]
    fn test_insert_delete_insert() {
        let mut avl = Avl::new();

        avl.insert(5, "five");
        avl.delete(&5);
        avl.insert(5, "FIVE");

        assert_eq!(avl.search(&5), Some("FIVE"));
    }

    #[test]
    fn test_large_tree() {
        let mut avl = Avl::new();

        // Insert 100 elements
        for i in 0..100 {
            avl.insert(i, i * 2);
        }

        // All should be searchable
        for i in 0..100 {
            assert_eq!(avl.search(&i), Some(i * 2));
        }

        // Height should be ~log2(100) ≈ 7
        let root = avl.root.as_ref().unwrap();
        assert!(root.borrow().height <= 10);
    }

    #[test]
    fn test_string_keys() {
        let mut avl = Avl::new();

        avl.insert("dog".to_string(), 1);
        avl.insert("cat".to_string(), 2);
        avl.insert("zebra".to_string(), 3);
        avl.insert("ant".to_string(), 4);

        assert_eq!(avl.search(&"dog".to_string()), Some(1));
        assert_eq!(avl.search(&"cat".to_string()), Some(2));

        avl.delete(&"cat".to_string());
        assert_eq!(avl.search(&"cat".to_string()), None);
    }

    #[test]
    fn test_random_operations() {
        let mut avl = Avl::new();

        avl.insert(50, "fifty");
        avl.insert(25, "twenty-five");
        avl.delete(&50);
        avl.insert(75, "seventy-five");
        avl.insert(10, "ten");
        avl.delete(&25);
        avl.insert(30, "thirty");

        assert_eq!(avl.search(&50), None);
        assert_eq!(avl.search(&25), None);
        assert_eq!(avl.search(&75), Some("seventy-five"));
        assert_eq!(avl.search(&10), Some("ten"));
        assert_eq!(avl.search(&30), Some("thirty"));
    }

    // Helper function to check balance from outside
    fn get_balance_from_outside<K: Ord + Clone, V: Clone>(
        node: &Rc<RefCell<AVLNode<K, V>>>,
    ) -> i32 {
        let borrowed = node.borrow();
        let left_height = borrowed
            .left
            .as_ref()
            .map(|n| n.borrow().height)
            .unwrap_or(0);
        let right_height = borrowed
            .right
            .as_ref()
            .map(|n| n.borrow().height)
            .unwrap_or(0);
        left_height - right_height
    }
}
