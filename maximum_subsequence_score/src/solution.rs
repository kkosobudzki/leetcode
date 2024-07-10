use std::collections::BinaryHeap;

pub fn max_score(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
    let mut pairs: Vec<(&i32, &i32)> = nums2.iter().zip(nums1.iter()).collect();
    pairs.sort_unstable_by(|a, b| b.cmp(a)); // descending

    let ku = k as usize;

    let slice = &pairs[..ku];

    let mut heap: BinaryHeap<i64> = slice.iter().map(|p| -*p.1 as i64).collect();
    let mut k_sum = slice.iter().fold(0i64, |sum, pair| sum + *pair.1 as i64);
    let mut max = *slice.last().unwrap().0 as i64 * k_sum;

    for i in ku..pairs.len() {
        let (&min, &num1) = pairs[i];

        heap.push(-num1 as i64);

        if let Some(smallest) = heap.pop() {
            k_sum += smallest as i64;
            k_sum += num1 as i64;

            max = max.max(min as i64 * k_sum);
        }
    }

    max
}

#[cfg(test)]
mod tests {
    use super::max_score;

    #[test]
    fn ex1() {
        let result = max_score(vec![1, 3, 3, 2], vec![2, 1, 3, 4], 3);

        assert_eq!(result, 12);
    }

    #[test]
    fn ex2() {
        let result = max_score(vec![4, 2, 3, 1, 1], vec![7, 5, 10, 9, 6], 1);

        assert_eq!(result, 30)
    }

    #[test]
    fn ex3() {
        let result = max_score(vec![2, 1, 14, 12], vec![11, 7, 13, 6], 3);

        assert_eq!(result, 168);
    }
}
