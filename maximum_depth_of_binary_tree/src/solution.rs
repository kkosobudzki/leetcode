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

    pub fn from_array(leafs: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        TreeNode::leaf_from(&leafs, 0)
    }

    fn leaf_from(leafs: &Vec<Option<i32>>, n: usize) -> Option<Rc<RefCell<TreeNode>>> {
        match leafs[n] {
            Some(val) => {
                let mut node = TreeNode::new(val);

                if 2 * n + 2 < leafs.len() {
                    node.left = TreeNode::leaf_from(leafs, 2 * n + 1);
                    node.right = TreeNode::leaf_from(leafs, 2 * n + 2);
                }

                Some(Rc::new(RefCell::new(node)))
            }
            None => None,
        }
    }
}

pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    max_depth_ref(&root)
}

fn max_depth_ref(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match root {
        Some(node) => {
            let left_depth = max_depth_ref(&node.borrow().left);
            let right_depth = max_depth_ref(&node.borrow().right);

            1 + cmp::max(left_depth, right_depth)
        }
        None => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::{max_depth, TreeNode};

    #[test]
    fn ex1() {
        let root = TreeNode::from_array(vec![
            Some(3),
            Some(9),
            Some(20),
            None,
            None,
            Some(15),
            Some(7),
        ]);

        let result = max_depth(root);

        assert_eq!(result, 3);
    }

    #[test]
    fn ex2() {
        let root = TreeNode::from_array(vec![Some(1), None, Some(2)]);

        let result = max_depth(root);

        assert_eq!(result, 2);
    }
}
