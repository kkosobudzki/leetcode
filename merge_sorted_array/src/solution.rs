pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let mut i = m - 1;
    let mut j = n - 1;
    let mut k = m + n - 1;

    while j >= 0 {
        if i >= 0 && nums1[i as usize] > nums2[j as usize] {
            nums1[k as usize] = nums1[i as usize];

            k -= 1;
            i -= 1;
        } else {
            nums1[k as usize] = nums2[j as usize];

            k -= 1;
            j -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::merge;

    #[test]
    fn ex1() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let mut nums2 = vec![2, 5, 6];

        merge(&mut nums1, 3, &mut nums2, 3);

        assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);
    }

    #[test]
    fn ex2() {
        let mut nums1 = vec![1];
        let mut nums2 = vec![];

        merge(&mut nums1, 1, &mut nums2, 0);

        assert_eq!(nums1, vec![1]);
    }

    #[test]
    fn ex3() {
        let mut nums1 = vec![0];
        let mut nums2 = vec![1];

        merge(&mut nums1, 0, &mut nums2, 1);

        assert_eq!(nums1, vec![1]);
    }

    #[test]
    fn ex4() {
        let mut nums1 = vec![4, 5, 6, 0, 0, 0];
        let mut nums2 = vec![1, 2, 3];

        merge(&mut nums1, 3, &mut nums2, 3);

        assert_eq!(nums1, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn ex5() {
        let mut nums1 = vec![1, 2, 4, 5, 6, 0];
        let mut nums2 = vec![3];

        merge(&mut nums1, 5, &mut nums2, 1);

        assert_eq!(nums1, vec![1, 2, 3, 4, 5, 6]);
    }
}
