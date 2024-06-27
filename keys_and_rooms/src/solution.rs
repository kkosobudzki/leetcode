use std::collections::{HashMap, VecDeque};

pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
    let mut visited = HashMap::new();
    let mut queue: VecDeque<i32> = VecDeque::new();

    queue.push_back(0);

    while let Some(key) = queue.pop_front() {
        visited.insert(key, true);

        for k in &rooms[key as usize] {
            if visited.get(k) == None {
                queue.push_back(*k);
            }
        }
    }

    visited.len() == rooms.len()
}

#[cfg(test)]
mod tests {
    use super::can_visit_all_rooms;

    #[test]
    fn ex1() {
        let result = can_visit_all_rooms(vec![vec![1], vec![2], vec![3], vec![]]);

        assert_eq!(result, true);
    }

    #[test]
    fn ex2() {
        let result = can_visit_all_rooms(vec![vec![1, 3], vec![3, 0, 1], vec![2], vec![0]]);

        assert_eq!(result, false);
    }
}
