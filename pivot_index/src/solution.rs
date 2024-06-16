pub fn pivot_index(nums: Vec<i32>) -> i32 {
    let length = nums.len();

    let mut left = vec![0];
    let mut right = vec![0];

    for i in 0..length {
        left.push(left[i] + nums[i]);
        right.push(right[i] + nums[length - i - 1]);
    }

    for i in 0..length {
        if left[i] == right[length - i - 1] {
            return i as i32;
        }
    }

    -1
}

#[cfg(test)]
mod tests {
    use super::pivot_index;

    #[test]
    fn ex1() {
        let nums = vec![1, 7, 3, 6, 5, 6];

        let result = pivot_index(nums);

        assert_eq!(result, 3);
    }

    #[test]
    fn ex2() {
        let nums = vec![1, 2, 3];

        let result = pivot_index(nums);

        assert_eq!(result, -1);
    }

    #[test]
    fn ex3() {
        let nums = vec![2, 1, -1];

        let result = pivot_index(nums);

        assert_eq!(result, 0);
    }
}
