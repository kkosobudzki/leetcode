use std::cell::RefCell;
use std::cmp::Ordering::{Equal, Greater, Less};
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

pub fn delete_node(root: Option<Rc<RefCell<TreeNode>>>, key: i32) -> Option<Rc<RefCell<TreeNode>>> {
    let node = root?;
    let mut node_ref = node.borrow_mut();

    match key.cmp(&node_ref.val) {
        Equal => {
            return match (node_ref.left.take(), node_ref.right.take()) {
                (None, None) => None,
                (Some(child), None) | (None, Some(child)) => Some(child.clone()),
                (Some(left), Some(right)) => {
                    let min = find_inorder_successor(&right).borrow().val;

                    let mut node = TreeNode::new(min);
                    node.left = Some(left.clone());
                    node.right = delete_node(Some(right), min);

                    Some(Rc::new(RefCell::new(node)))
                }
            }
        }
        Less => node_ref.left = delete_node(node_ref.left.take(), key),
        Greater => node_ref.right = delete_node(node_ref.right.take(), key),
    }

    Some(node.clone())
}

fn find_inorder_successor(root: &Rc<RefCell<TreeNode>>) -> Rc<RefCell<TreeNode>> {
    match &root.borrow().left {
        Some(left) => find_inorder_successor(left),
        None => root.clone(),
    }
}
