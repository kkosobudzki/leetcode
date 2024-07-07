use std::cmp::max;

pub fn rob(nums: Vec<i32>) -> i32 {
    match nums.len() {
        0 => return 0,
        1 => return nums[0],
        2 => return max(nums[0], nums[1]),
        _ => {}
    }

    let mut dp = Vec::new();
    dp.push(nums[0]);
    dp.push(max(nums[0], nums[1]));

    for i in 2..nums.len() {
        dp.push(max(dp[i - 2] + nums[i], dp[i - 1]));
    }

    *dp.last().unwrap()
}

#[cfg(test)]
mod tests {
    use super::rob;

    #[test]
    fn ex1() {
        let result = rob(vec![1, 2, 3, 1]);

        assert_eq!(result, 4);
    }

    #[test]
    fn ex2() {
        let result = rob(vec![2, 7, 9, 3, 1]);

        assert_eq!(result, 12);
    }

    #[test]
    fn ex3() {
        let result = rob(vec![2, 1, 1, 2]);

        assert_eq!(result, 4);
    }
}
