use std::{cell::RefCell, rc::Rc};

pub struct TreeNode {
    pub character: Option<char>,
    pub frequency: usize,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    // Leaf, must have a char.
    pub fn new_leaf(character: char, frequency: usize) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            character: Some(character),
            frequency,
            left: None,
            right: None,
        }))
    }

    // internal node, no char just connects to other nodes
    pub fn new_internal(
        frequency: usize,
        left: Rc<RefCell<TreeNode>>,
        right: Rc<RefCell<TreeNode>>,
    ) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            character: None,
            frequency,
            left: Some(left),
            right: Some(right),
        }))
    }

    pub fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }
}
