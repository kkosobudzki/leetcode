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

pub fn search_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
    search_bst_ref(&root, val)
}

fn search_bst_ref(root: &Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
    match root {
        Some(node) => {
            let borrowed = node.borrow();

            match borrowed.val {
                v if v == val => root.to_owned(),
                v if v > val => search_bst_ref(&borrowed.left, val),
                _ => search_bst_ref(&borrowed.right, val),
            }
        }
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use super::{search_bst, TreeNode};

    #[test]
    fn ex1() {
        let root = TreeNode::from_array(vec![Some(4), Some(2), Some(7), Some(1), Some(3)]);

        let result = search_bst(root, 2);

        assert_eq!(result.unwrap().borrow().val, 2);
    }

    #[test]
    fn ex2() {
        let root = TreeNode::from_array(vec![Some(4), Some(2), Some(7), Some(1), Some(3)]);

        let result = search_bst(root, 5);

        assert_eq!(result, None);
    }
}
