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

    fn from_iter(values: Vec<i32>) -> Option<Box<ListNode>> {
        values.iter().rev().fold(None, |next, &val| {
            let mut node = ListNode::new(val);
            node.next = next;

            Some(Box::new(node))
        })
    }

    fn as_vec(self) -> Vec<i32> {
        let mut values = Vec::new();
        values.push(self.val);

        let mut node = self;

        while let Some(n) = node.next {
            values.push(n.val);

            node = *n;
        }

        values
    }
}

pub fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut current = head.clone();

    let mut odd = Box::new(ListNode::new(1));
    let mut even = Box::new(ListNode::new(2));

    let mut odd_tail = &mut odd;
    let mut even_tail = &mut even;

    let mut index = 1;

    while let Some(mut node) = current {
        current = node.next.take();

        match index % 2 {
            0 => {
                even_tail.next = Some(node);
                even_tail = even_tail.next.as_mut()?;
            }
            1 => {
                odd_tail.next = Some(node);
                odd_tail = odd_tail.next.as_mut()?;
            }
            _ => unreachable!(),
        }

        index += 1;
    }

    odd_tail.next = even.next;

    odd.next
}

#[cfg(test)]
mod tests {
    use super::{odd_even_list, ListNode};

    #[test]
    fn ex1() {
        let head = ListNode::from_iter(vec![1, 2, 3, 4, 5]);

        let result = odd_even_list(head);

        println!("ex1 result: {:?}", result);

        assert_eq!(result.unwrap().as_vec(), vec![1, 3, 5, 2, 4]);
    }

    #[test]
    fn ex2() {
        let head = ListNode::from_iter(vec![2, 1, 3, 5, 6, 4, 7]);

        let result = odd_even_list(head);

        assert_eq!(result.unwrap().as_vec(), vec![2, 3, 6, 7, 1, 5, 4]);
    }
}
