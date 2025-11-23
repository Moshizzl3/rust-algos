// Binary Search Tree (Bst)

use core::borrow;
use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub struct BstNode<K, V> {
    pub key: K,
    pub value: V,
    pub left: Option<Rc<RefCell<BstNode<K, V>>>>,
    pub right: Option<Rc<RefCell<BstNode<K, V>>>>,
}

#[derive(Debug)]
pub struct Bst<K, V> {
    pub root: Option<Rc<RefCell<BstNode<K, V>>>>,
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

    pub fn search(&self, key: &K) -> Option<V>
    where
        V: Clone,
    {
        if let Some(ref node) = self.root {
            Self::search_helper(node, key)
        } else {
            None
        }
    }
    fn search_helper(node: &Rc<RefCell<BstNode<K, V>>>, key: &K) -> Option<V>
    where
        V: Clone,
    {
        let borrowed = node.borrow();
        let left_node = borrowed.left.clone();
        let right_node = borrowed.right.clone();
        if *key == borrowed.key {
            return Some(borrowed.value.clone());
        }
        if *key < borrowed.key {
            // Go left here to check left side
            if let Some(ref left) = left_node {
                drop(borrowed);
                Self::search_helper(left, key)
            } else {
                None // return none if not found.
            }
        } else if let Some(ref right) = right_node {
            drop(borrowed);
            Self::search_helper(right, key)
        } else {
            None
        }
    }
    pub fn delete(&mut self, key: &K)
    where
        K: Clone,
        V: Clone,
    {
        if let Some(ref root) = self.root {
            self.root = Self::delete_helper(root, key)
        }
    }

    pub fn delete_helper(
        node: &Rc<RefCell<BstNode<K, V>>>,
        key: &K,
    ) -> Option<Rc<RefCell<BstNode<K, V>>>>
    where
        K: Clone,
        V: Clone,
    {
        let mut borrowed = node.borrow_mut();

        if *key < borrowed.key {
            if let Some(ref left) = borrowed.left.take() {
                borrowed.left = Self::delete_helper(left, key);
            }
        } else if *key > borrowed.key {
            if let Some(ref right) = borrowed.right.take() {
                borrowed.right = Self::delete_helper(right, key);
            }
        } else {
            // Is leaf (no children)
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
            borrowed.right = Self::delete_helper(&right, &borrowed.key)
        }

        Some(node.clone())
    }

    fn find_min(node: &Rc<RefCell<BstNode<K, V>>>) -> (K, V)
    where
        K: Clone,
        V: Clone,
    {
        let borrowed = node.borrow();
        if let Some(ref left) = borrowed.left {
            return Self::find_min(left);
        }
        (borrowed.key.clone(), borrowed.value.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_bst_is_empty() {
        let bst: Bst<i32, String> = Bst::new();
        assert!(bst.root.is_none());
    }

    #[test]
    fn test_insert_root() {
        let mut bst = Bst::new();
        bst.insert(5, "five");

        assert!(bst.root.is_some());
        let root = bst.root.as_ref().unwrap();
        assert_eq!(root.borrow().key, 5);
        assert_eq!(root.borrow().value, "five");
    }

    #[test]
    fn test_insert_multiple() {
        let mut bst = Bst::new();
        bst.insert(5, "five");
        bst.insert(3, "three");
        bst.insert(7, "seven");

        let root = bst.root.as_ref().unwrap();
        assert_eq!(root.borrow().key, 5);
        assert!(root.borrow().left.is_some());
        assert!(root.borrow().right.is_some());
    }

    #[test]
    fn test_insert_maintains_bst_property() {
        let mut bst = Bst::new();
        bst.insert(5, "five");
        bst.insert(3, "three");
        bst.insert(7, "seven");
        bst.insert(1, "one");
        bst.insert(9, "nine");

        let root = bst.root.as_ref().unwrap();
        let left = root.borrow().left.as_ref().unwrap().clone();
        let right = root.borrow().right.as_ref().unwrap().clone();

        // Left child < root
        assert!(left.borrow().key < root.borrow().key);
        // Right child > root
        assert!(right.borrow().key > root.borrow().key);
    }

    #[test]
    fn test_insert_updates_existing_key() {
        let mut bst = Bst::new();
        bst.insert(5, "five");
        bst.insert(5, "FIVE");

        let result = bst.search(&5);
        assert_eq!(result, Some("FIVE"));
    }

    #[test]
    fn test_search_finds_root() {
        let mut bst = Bst::new();
        bst.insert(5, "five");

        let result = bst.search(&5);
        assert_eq!(result, Some("five"));
    }

    #[test]
    fn test_search_finds_left_child() {
        let mut bst = Bst::new();
        bst.insert(5, "five");
        bst.insert(3, "three");

        let result = bst.search(&3);
        assert_eq!(result, Some("three"));
    }

    #[test]
    fn test_search_finds_right_child() {
        let mut bst = Bst::new();
        bst.insert(5, "five");
        bst.insert(7, "seven");

        let result = bst.search(&7);
        assert_eq!(result, Some("seven"));
    }

    #[test]
    fn test_search_finds_deep_node() {
        let mut bst = Bst::new();
        bst.insert(5, "five");
        bst.insert(3, "three");
        bst.insert(7, "seven");
        bst.insert(1, "one");
        bst.insert(9, "nine");

        assert_eq!(bst.search(&1), Some("one"));
        assert_eq!(bst.search(&9), Some("nine"));
    }

    #[test]
    fn test_search_not_found() {
        let mut bst = Bst::new();
        bst.insert(5, "five");
        bst.insert(3, "three");

        let result = bst.search(&10);
        assert_eq!(result, None);
    }

    #[test]
    fn test_search_empty_tree() {
        let bst: Bst<i32, String> = Bst::new();
        assert_eq!(bst.search(&5), None);
    }

    #[test]
    fn test_multiple_inserts_and_searches() {
        let mut bst = Bst::new();
        let pairs = vec![
            (8, "eight"),
            (3, "three"),
            (10, "ten"),
            (1, "one"),
            (6, "six"),
            (14, "fourteen"),
            (4, "four"),
            (7, "seven"),
            (13, "thirteen"),
        ];

        for (key, value) in &pairs {
            bst.insert(*key, *value);
        }

        for (key, value) in &pairs {
            assert_eq!(bst.search(key), Some(*value));
        }
    }

    #[test]
    fn test_insert_ascending_order() {
        let mut bst = Bst::new();
        for i in 1..=5 {
            bst.insert(i, i * 10);
        }

        // All should be found
        for i in 1..=5 {
            assert_eq!(bst.search(&i), Some(i * 10));
        }
    }

    #[test]
    fn test_insert_descending_order() {
        let mut bst = Bst::new();
        for i in (1..=5).rev() {
            bst.insert(i, i * 10);
        }

        // All should be found
        for i in 1..=5 {
            assert_eq!(bst.search(&i), Some(i * 10));
        }
    }

    #[test]
    fn test_search_with_string_keys() {
        let mut bst = Bst::new();
        bst.insert("dog".to_string(), 1);
        bst.insert("cat".to_string(), 2);
        bst.insert("zebra".to_string(), 3);

        assert_eq!(bst.search(&"dog".to_string()), Some(1));
        assert_eq!(bst.search(&"cat".to_string()), Some(2));
        assert_eq!(bst.search(&"zebra".to_string()), Some(3));
        assert_eq!(bst.search(&"bird".to_string()), None);
    }

    #[test]
    fn test_large_tree() {
        let mut bst = Bst::new();

        // Insert 100 elements
        for i in 0..100 {
            bst.insert(i, i * 2);
        }

        // Search for all of them
        for i in 0..100 {
            assert_eq!(bst.search(&i), Some(i * 2));
        }

        // Search for non-existent
        assert_eq!(bst.search(&100), None);
        assert_eq!(bst.search(&-1), None);
    }

    // test delete
    #[test]
    fn test_delete_leaf() {
        let mut bst = Bst::new();
        bst.insert(5, "five");
        bst.insert(3, "three");
        bst.insert(7, "seven");
        bst.insert(1, "one");

        bst.delete(&1);

        assert_eq!(bst.search(&1), None);
        assert_eq!(bst.search(&3), Some("three"));
        assert_eq!(bst.search(&5), Some("five"));
    }

    #[test]
    fn test_delete_node_with_left_child_only() {
        let mut bst = Bst::new();
        bst.insert(5, "five");
        bst.insert(3, "three");
        bst.insert(1, "one");

        bst.delete(&3); // 3 has only left child (1)

        assert_eq!(bst.search(&3), None);
        assert_eq!(bst.search(&1), Some("one"));
        assert_eq!(bst.search(&5), Some("five"));
    }

    #[test]
    fn test_delete_node_with_right_child_only() {
        let mut bst = Bst::new();
        bst.insert(5, "five");
        bst.insert(3, "three");
        bst.insert(4, "four");

        bst.delete(&3); // 3 has only right child (4)

        assert_eq!(bst.search(&3), None);
        assert_eq!(bst.search(&4), Some("four"));
        assert_eq!(bst.search(&5), Some("five"));
    }

    #[test]
    fn test_delete_node_with_two_children() {
        let mut bst = Bst::new();
        bst.insert(5, "five");
        bst.insert(3, "three");
        bst.insert(7, "seven");
        bst.insert(1, "one");
        bst.insert(4, "four");
        bst.insert(6, "six");
        bst.insert(9, "nine");

        bst.delete(&5); // Root with two children

        assert_eq!(bst.search(&5), None);
        // All others should still exist
        assert_eq!(bst.search(&3), Some("three"));
        assert_eq!(bst.search(&7), Some("seven"));
        assert_eq!(bst.search(&6), Some("six"));
    }

    #[test]
    fn test_delete_root_leaf() {
        let mut bst = Bst::new();
        bst.insert(5, "five");

        bst.delete(&5);

        assert_eq!(bst.search(&5), None);
    }

    #[test]
    fn test_delete_nonexistent() {
        let mut bst = Bst::new();
        bst.insert(5, "five");
        bst.insert(3, "three");

        bst.delete(&10); // Doesn't exist

        // Nothing should change
        assert_eq!(bst.search(&5), Some("five"));
        assert_eq!(bst.search(&3), Some("three"));
    }

    #[test]
    fn test_delete_multiple() {
        let mut bst = Bst::new();
        for i in 1..=10 {
            bst.insert(i, i * 10);
        }

        bst.delete(&5);
        bst.delete(&3);
        bst.delete(&8);

        assert_eq!(bst.search(&5), None);
        assert_eq!(bst.search(&3), None);
        assert_eq!(bst.search(&8), None);

        assert_eq!(bst.search(&1), Some(10));
        assert_eq!(bst.search(&7), Some(70));
    }

    #[test]
    fn test_delete_all_nodes() {
        let mut bst = Bst::new();
        bst.insert(5, "five");
        bst.insert(3, "three");
        bst.insert(7, "seven");

        bst.delete(&5);
        bst.delete(&3);
        bst.delete(&7);

        assert_eq!(bst.search(&5), None);
        assert_eq!(bst.search(&3), None);
        assert_eq!(bst.search(&7), None);
    }

    #[test]
    fn test_delete_and_reinsert() {
        let mut bst = Bst::new();
        bst.insert(5, "five");

        bst.delete(&5);
        assert_eq!(bst.search(&5), None);

        bst.insert(5, "FIVE");
        assert_eq!(bst.search(&5), Some("FIVE"));
    }

    // String key tests
    #[test]
    fn test_delete_with_string_keys() {
        let mut bst = Bst::new();
        bst.insert("dog".to_string(), 1);
        bst.insert("cat".to_string(), 2);
        bst.insert("zebra".to_string(), 3);
        bst.insert("ant".to_string(), 4);

        bst.delete(&"cat".to_string());

        assert_eq!(bst.search(&"cat".to_string()), None);
        assert_eq!(bst.search(&"dog".to_string()), Some(1));
        assert_eq!(bst.search(&"zebra".to_string()), Some(3));
    }

    #[test]
    fn test_delete_string_node_with_two_children() {
        let mut bst = Bst::new();
        bst.insert("m".to_string(), 1);
        bst.insert("d".to_string(), 2);
        bst.insert("s".to_string(), 3);
        bst.insert("a".to_string(), 4);
        bst.insert("h".to_string(), 5);
        bst.insert("p".to_string(), 6);
        bst.insert("z".to_string(), 7);

        bst.delete(&"m".to_string()); // Root with two children

        assert_eq!(bst.search(&"m".to_string()), None);
        assert_eq!(bst.search(&"d".to_string()), Some(2));
        assert_eq!(bst.search(&"s".to_string()), Some(3));
        assert_eq!(bst.search(&"p".to_string()), Some(6));
    }

    #[test]
    fn test_complex_deletions() {
        let mut bst = Bst::new();
        //       8
        //      / \
        //     3   10
        //    / \    \
        //   1   6   14
        //      / \  /
        //     4  7 13

        bst.insert(8, "eight");
        bst.insert(3, "three");
        bst.insert(10, "ten");
        bst.insert(1, "one");
        bst.insert(6, "six");
        bst.insert(14, "fourteen");
        bst.insert(4, "four");
        bst.insert(7, "seven");
        bst.insert(13, "thirteen");

        // Delete node with two children
        bst.delete(&3);
        assert_eq!(bst.search(&3), None);
        assert_eq!(bst.search(&4), Some("four")); // Successor should exist

        // Delete another node with two children
        bst.delete(&10);
        assert_eq!(bst.search(&10), None);

        // All others should still exist
        assert_eq!(bst.search(&8), Some("eight"));
        assert_eq!(bst.search(&1), Some("one"));
    }
}
