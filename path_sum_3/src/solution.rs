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

pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
    dfs(&root, &vec![0], target_sum)
}

fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, prefix_sum: &Vec<i32>, target_sum: i32) -> i32 {
    match node.as_ref() {
        Some(rc) => {
            let n = rc.borrow();

            let mut ps = prefix_sum.clone();
            ps.push(n.val);

            let mut i = ps.len() - 1;
            let mut sum: i64 = 0;

            let mut count = 0;

            while i > 0 {
                sum += ps[i] as i64;

                if sum == target_sum as i64 {
                    count += 1;
                }

                i -= 1;
            }

            count + dfs(&n.left, &ps, target_sum) + dfs(&n.right, &ps, target_sum)
        }
        None => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::path_sum;
    use super::TreeNode;

    #[test]
    fn ex1() {
        let tree = vec![
            Some(10),
            Some(5),
            Some(-3),
            Some(3),
            Some(2),
            None,
            Some(11),
            Some(3),
            Some(-2),
            None,
            Some(1),
        ];

        let result = path_sum(TreeNode::from_vec(tree), 8);

        assert_eq!(result, 3);
    }

    #[test]
    fn ex2() {
        let tree = vec![
            Some(5),
            Some(4),
            Some(8),
            Some(11),
            None,
            Some(13),
            Some(4),
            Some(7),
            Some(2),
            None,
            None,
            Some(5),
            Some(1),
        ];

        let result = path_sum(TreeNode::from_vec(tree), 22);

        assert_eq!(result, 3);
    }

    #[test]
    fn ex3() {
        let tree = vec![Some(0), Some(1), Some(1)];

        let result = path_sum(TreeNode::from_vec(tree), 1);

        assert_eq!(result, 4);
    }

    #[test]
    fn ex4() {
        let tree = vec![
            Some(1000000000),
            Some(1000000000),
            None,
            Some(294967296),
            None,
            Some(1000000000),
            None,
            Some(1000000000),
            None,
            Some(1000000000),
        ];

        let result = path_sum(TreeNode::from_vec(tree), 0);

        assert_eq!(result, 0);
    }
}
