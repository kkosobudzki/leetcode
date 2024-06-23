#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    fn from_vec(values: Vec<i32>) -> Option<Box<ListNode>> {
        values.iter().rev().fold(None, |next, &val| {
            let mut node = ListNode::new(val);
            node.next = next;

            Some(Box::new(node))
        })
    }
}

pub fn pair_sum(head: Option<Box<ListNode>>) -> i32 {
    let mut values: Vec<i32> = Vec::new();

    let mut slow = head.as_ref();
    let mut fast = head.as_ref();

    while let Some(node) = fast {
        match node.next.as_ref() {
            Some(next) => {
                fast = next.next.as_ref();
                slow = slow.and_then(|node| {
                    values.push(node.val);

                    node.next.as_ref()
                });
            }
            None => break,
        }
    }

    let mut max = 0;

    while let Some(node) = slow {
        if let Some(twin) = values.pop() {
            let sum = node.val + twin;

            if sum > max {
                max = sum
            }
        }

        slow = node.next.as_ref();
    }

    max
}

#[cfg(test)]
mod tests {
    use super::{pair_sum, ListNode};

    #[test]
    fn ex1() {
        let head = ListNode::from_vec(vec![5, 4, 2, 1]);

        let result = pair_sum(head);

        assert_eq!(result, 6);
    }

    #[test]
    fn ex2() {
        let head = ListNode::from_vec(vec![4, 2, 2, 3]);

        let result = pair_sum(head);

        assert_eq!(result, 7);
    }

    #[test]
    fn ex3() {
        let head = ListNode::from_vec(vec![1, 100_000]);

        let result = pair_sum(head);

        assert_eq!(result, 100_001);
    }
}
