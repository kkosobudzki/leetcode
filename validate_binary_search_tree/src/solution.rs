use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    is_valid(root.as_ref(), i32::MIN as i64 - 1, i32::MAX as i64 + 1)
}

fn is_valid(node: Option<&Rc<RefCell<TreeNode>>>, left: i64, right: i64) -> bool {
    match node {
        None => true,
        Some(rc) => {
            let borrowed = rc.borrow();
            let value = borrowed.val as i64;

            value > left
                && value < right
                && is_valid(borrowed.left.as_ref(), left, value)
                && is_valid(borrowed.right.as_ref(), value, right)
        }
    }
}
