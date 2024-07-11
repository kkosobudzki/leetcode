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

pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut result = Vec::new();

    inorder(root.as_ref(), &mut result);

    result
}

fn inorder(node: Option<&Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
    if let Some(rc) = node {
        let borrowed = rc.borrow();

        inorder(borrowed.left.as_ref(), result);

        result.push(borrowed.val);

        inorder(borrowed.right.as_ref(), result);
    }
}
