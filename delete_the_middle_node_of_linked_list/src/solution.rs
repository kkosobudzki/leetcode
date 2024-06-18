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

    fn from_array(mut arr: Vec<i32>) -> Option<Box<ListNode>> {
        match arr.pop() {
            Some(val) => arr
                .iter()
                .rev()
                .fold(Some(Box::new(ListNode::new(val))), |next, &v| {
                    let mut node = ListNode::new(v);
                    node.next = next;

                    Some(Box::new(node))
                }),
            None => None,
        }
    }

    fn to_array(head: &Option<Box<ListNode>>) -> Vec<i32> {
        let mut arr = Vec::new();

        let mut current = head;

        loop {
            match current {
                Some(boxed) => {
                    arr.push(boxed.val);

                    current = &boxed.next;
                }
                None => break,
            }
        }

        arr
    }
}

pub fn delete_middle(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    // because head is immutable, new head has to be created and mutated
    // then join with the old list
    let mut new_head = head.clone();
    let mut previous = new_head.as_mut();

    if previous.as_ref().unwrap().next.is_none() {
        return None;
    }

    let mut slow = head.as_ref();
    let mut fast = head.as_ref();

    let mut skip = true;

    while let Some(node) = fast {
        match node.next.as_ref() {
            Some(next_node) => {
                if skip {
                    skip = false
                } else {
                    previous = previous.and_then(|node| node.next.as_mut());
                }

                fast = next_node.next.as_ref();
                slow = slow.and_then(|node| node.next.as_ref());
            }
            None => break,
        }
    }

    previous.unwrap().next = slow.unwrap().next.clone();

    new_head
}

#[cfg(test)]
mod tests {
    use super::{delete_middle, ListNode};

    #[test]
    pub fn ex1() {
        let list = vec![1, 3, 4, 7, 1, 2, 6];
        let head = ListNode::from_array(list);

        let result = delete_middle(head);

        assert_eq!(ListNode::to_array(&result), vec![1, 3, 4, 1, 2, 6])
    }

    #[test]
    pub fn ex2() {
        let list = vec![1, 2, 3, 4];
        let head = ListNode::from_array(list);

        let result = delete_middle(head);

        assert_eq!(ListNode::to_array(&result), vec![1, 2, 4])
    }

    #[test]
    pub fn ex3() {
        let list = vec![2, 1];
        let head = ListNode::from_array(list);

        let result = delete_middle(head);

        assert_eq!(ListNode::to_array(&result), vec![2])
    }

    #[test]
    pub fn ex4_single_element() {
        let list = vec![1];
        let head = ListNode::from_array(list);

        let result = delete_middle(head);

        assert_eq!(ListNode::to_array(&result), vec![])
    }
}
