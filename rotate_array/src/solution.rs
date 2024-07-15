pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    let pivot = nums.len() - k as usize % nums.len();

    let left = &nums[..pivot].to_vec();
    let right = &nums[pivot..].to_vec();

    for i in 0..right.len() {
        nums[i] = right[i];
    }

    for i in 0..left.len() {
        nums[i + right.len()] = left[i];
    }
}

#[cfg(test)]
mod tests {
    use super::rotate;

    #[test]
    fn ex1() {
        let mut nums = vec![1, 2, 3, 4, 5, 6, 7];

        rotate(&mut nums, 3);

        assert_eq!(nums, vec![5, 6, 7, 1, 2, 3, 4]);
    }

    #[test]
    fn ex2() {
        let mut nums = vec![-1, -100, 3, 99];

        rotate(&mut nums, 2);

        assert_eq!(nums, vec![3, 99, -1, -100]);
    }
}
