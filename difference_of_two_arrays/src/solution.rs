use std::collections::HashSet;

pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
    let set1: HashSet<i32> = HashSet::from_iter(nums1);
    let set2: HashSet<i32> = HashSet::from_iter(nums2);

    let diff1 = set1.difference(&set2).cloned().collect();
    let diff2 = set2.difference(&set1).cloned().collect();

    vec![diff1, diff2]
}

#[cfg(test)]
mod tests {
    use super::find_difference;

    #[test]
    fn ex1() {
        let nums1 = vec![1, 2, 3];
        let nums2 = vec![2, 4, 6];

        let result = find_difference(nums1, nums2);

        assert_eq!(result, [vec![3, 1], vec![6, 4]]);
    }

    #[test]
    fn ex2() {
        let nums1 = vec![1, 2, 3, 3];
        let nums2 = vec![1, 1, 2, 2];

        let result = find_difference(nums1, nums2);

        assert_eq!(result, [vec![3], vec![]]);
    }
}
