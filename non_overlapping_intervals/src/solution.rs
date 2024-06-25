pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
    if intervals.len() < 2 {
        return 0;
    }

    intervals.sort_unstable();

    let mut to_remove = 0;

    let mut previous = &intervals[0];

    for i in 1..intervals.len() {
        let current = &intervals[i];

        if previous[0] <= current[0] && previous[1] >= current[1] {
            // previous contains current
            to_remove += 1;

            previous = current;
        } else if previous[1] > current[0] && previous[1] < current[1] {
            // partially overlaps
            to_remove += 1;
        } else {
            previous = current;
        }
    }

    to_remove
}

#[cfg(test)]
mod tests {
    use super::erase_overlap_intervals;

    #[test]
    fn ex1() {
        let intervals = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 3]];

        let result = erase_overlap_intervals(intervals);

        assert_eq!(result, 1);
    }

    #[test]
    fn ex2() {
        let intervals = vec![vec![1, 2], vec![1, 2], vec![1, 2]];

        let result = erase_overlap_intervals(intervals);

        assert_eq!(result, 2);
    }

    #[test]
    fn ex3() {
        let intervals = vec![vec![1, 2], vec![2, 3]];

        let result = erase_overlap_intervals(intervals);

        assert_eq!(result, 0);
    }

    #[test]
    fn ex4() {
        let intervals = vec![vec![1, 100], vec![11, 22], vec![1, 11], vec![2, 12]];

        let result = erase_overlap_intervals(intervals);

        assert_eq!(result, 2);
    }
}
