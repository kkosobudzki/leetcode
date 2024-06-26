use std::cell::RefCell;
use std::cmp;
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

    pub fn from_vec(values: Vec<Option<i32>>) -> Option<Rc<RefCell<Self>>> {
        Self::build(&values, 0)
    }

    fn build(values: &Vec<Option<i32>>, index: usize) -> Option<Rc<RefCell<Self>>> {
        if index >= values.len() {
            return None;
        }

        values[index].map(|v| {
            let mut node = Self::new(v);
            node.left = Self::build(values, 2 * index + 1);
            node.right = Self::build(values, 2 * index + 2);

            Rc::new(RefCell::new(node))
        })
    }
}

pub fn longest_zig_zag(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    cmp::max(length(&root, true, 0), length(&root, false, 0)) - 1
}

fn length(root: &Option<Rc<RefCell<TreeNode>>>, is_left: bool, depth: i32) -> i32 {
    match root.as_ref() {
        Some(rc) => {
            let node = rc.borrow();

            if is_left {
                cmp::max(
                    length(&node.left, false, depth + 1),
                    length(&node.left, true, 0),
                )
            } else {
                cmp::max(
                    length(&node.right, true, depth + 1),
                    length(&node.right, false, 0),
                )
            }
        }
        None => depth,
    }
}
