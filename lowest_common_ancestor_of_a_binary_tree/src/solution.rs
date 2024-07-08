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

pub fn lowest_common_ancestor(
    root: Option<Rc<RefCell<TreeNode>>>,
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    if root.is_none() {
        return None;
    }

    let root_ref = root.as_ref().unwrap();
    let value = root_ref.borrow().val;

    if let Some(rc) = &p {
        if rc.borrow().val == value {
            return root;
        }
    }

    if let Some(rc) = &q {
        if rc.borrow().val == value {
            return root;
        }
    }

    let left = lowest_common_ancestor(root_ref.borrow_mut().left.take(), p.clone(), q.clone());
    let right = lowest_common_ancestor(root_ref.borrow_mut().right.take(), p.clone(), q.clone());

    if left.is_none() {
        return right;
    } else if right.is_none() {
        return left;
    }

    root
}
