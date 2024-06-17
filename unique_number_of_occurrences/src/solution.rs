use std::collections::{HashMap, HashSet};

pub fn unique_occurrences(arr: Vec<i32>) -> bool {
    let mut map = HashMap::new();

    for num in arr {
        let count = match map.get(&num) {
            Some(count) => count + 1,
            None => 1,
        };

        map.insert(num, count);
    }

    let mut set = HashSet::new();

    for num in map.values() {
        if set.contains(num) {
            return false;
        }

        set.insert(num);
    }

    true
}

#[cfg(test)]
mod tests {
    use super::unique_occurrences;

    #[test]
    fn ex1() {
        let result = unique_occurrences(vec![1, 2, 2, 1, 1, 3]);

        assert_eq!(result, true);
    }

    #[test]
    fn ex2() {
        let result = unique_occurrences(vec![1, 2]);

        assert_eq!(result, false);
    }

    #[test]
    fn ex3() {
        let result = unique_occurrences(vec![-3, 0, 1, -3, 1, 1, 1, -3, 10, 0]);

        assert_eq!(result, true);
    }
}
