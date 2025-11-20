use std::{cell::RefCell, rc::Rc};

struct TreeNode {
    character: Option<char>,
    frequency: usize,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

