use std::cell::RefCell;
use std::collections::VecDeque;
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

pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    bfs(root)
}

fn bfs(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut values = Vec::new();

    if root.is_none() {
        return vec![];
    }

    let mut queue = VecDeque::new();
    queue.push_back(root.unwrap());

    while !queue.is_empty() {
        let length = queue.len();

        for i in 0..length {
            if let Some(rc) = queue.pop_front() {
                let node = rc.borrow();

                if i == length - 1 {
                    values.push(node.val);
                }

                if let Some(left) = &node.left {
                    queue.push_back(left.clone());
                }

                if let Some(right) = &node.right {
                    queue.push_back(right.clone());
                }
            }
        }
    }

    values
}
