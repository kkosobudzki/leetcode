use std::collections::VecDeque;

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
}

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut current: Option<Box<ListNode>> = head;

    let mut queue = VecDeque::new();

    loop {
        match current {
            Some(boxed) => {
                queue.push_back(boxed.val);

                current = boxed.next;
            }
            None => break,
        }
    }

    queue.iter().fold(None, |next, &value| {
        let mut node = ListNode::new(value);
        node.next = next;

        Some(Box::new(node))
    })
}

// #[cfg(test)]
// mod tests {
//     use super::ListNode;

//     #[test]
//     fn ex1() {
//         let mut next = ListNode::new(5);

//         for i in 4..1 {
//             let mut node = ListNode::new(i);
//             node.link(next);

//             next = node;
//         }

//         let
//     }
// }
