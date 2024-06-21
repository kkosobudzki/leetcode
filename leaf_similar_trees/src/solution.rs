use std::cell::RefCell;
use std::rc::Rc;
use std::vec;

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

pub fn leaf_similar(
    root1: Option<Rc<RefCell<TreeNode>>>,
    root2: Option<Rc<RefCell<TreeNode>>>,
) -> bool {
    collect_leafs(&root1) == collect_leafs(&root2)
}

fn collect_leafs(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut leafs = vec![];

    if let Some(node) = root {
        let node_ref = node.borrow();

        if node_ref.left.is_none() && node_ref.right.is_none() {
            leafs.push(node_ref.val);
        } else {
            leafs.extend(collect_leafs(&node_ref.left));
            leafs.extend(collect_leafs(&node_ref.right));
        }
    }

    leafs
}
