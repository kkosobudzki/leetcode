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

pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    dfs(&root, std::i32::MIN)
}

fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, max: i32) -> i32 {
    match node {
        Some(rc) => {
            let n = rc.borrow();

            if n.val >= max {
                dfs(&n.left, n.val) + dfs(&n.right, n.val) + 1
            } else {
                dfs(&n.left, max) + dfs(&n.right, max)
            }
        }
        None => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::{good_nodes, TreeNode};

    #[test]
    fn ex1() {
        let tree = vec![Some(3), Some(1), Some(4), Some(3), None, Some(1), Some(5)];

        let result = good_nodes(TreeNode::from_vec(tree));

        assert_eq!(result, 4);
    }

    #[test]
    fn ex2() {
        let tree = vec![Some(3), Some(3), None, Some(4), Some(2)];

        let result = good_nodes(TreeNode::from_vec(tree));

        assert_eq!(result, 3);
    }

    #[test]
    fn ex3() {
        let tree = vec![Some(1)];

        let result = good_nodes(TreeNode::from_vec(tree));

        assert_eq!(result, 1);
    }
}
