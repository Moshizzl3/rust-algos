// AVL (Adelson-Velsky and Landis) binary search tree, self balancing tree,
// goal is to have O(log n) search time aka height.

use core::{borrow, fmt};
use std::{cell::RefCell, fmt::Display, rc::Rc};

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
