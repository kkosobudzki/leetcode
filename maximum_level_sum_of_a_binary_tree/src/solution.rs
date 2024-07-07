use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
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

pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match root {
        Some(rc) => bfs(rc),
        None => 0,
    }
}

#[inline]
fn bfs(root: Rc<RefCell<TreeNode>>) -> i32 {
    let mut level = 1;
    let mut max = (root.borrow().val, level);

    let mut queue = VecDeque::new();
    queue.push_back(root);

    while !queue.is_empty() {
        let length = queue.len();

        let mut sum = 0;

        for i in 0..length {
            if let Some(rc) = queue.pop_front() {
                let node = rc.borrow();

                sum += node.val;

                if i == length - 1 {
                    if sum > max.0 {
                        max = (sum, level);
                    }

                    level += 1;
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

    max.0
}
