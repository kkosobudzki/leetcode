pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
    let ku = k as usize;

    let mut sum = nums[0..ku].iter().fold(0.0, |acc, &num| acc + num as f64);
    let mut max_sum = sum;

    for i in 1..nums.len() - ku + 1 {
        sum = sum - nums[i - 1] as f64 + nums[i + ku - 1] as f64;

        if sum > max_sum {
            max_sum = sum
        }
    }

    max_sum / k as f64
}

#[cfg(test)]
mod tests {
    use super::find_max_average;

    #[test]
    fn ex1() {
        let nums = vec![1, 12, -5, -6, 50, 3];
        let k = 4;

        let result = find_max_average(nums, k);

        assert_eq!(result, 12.75);
    }

    #[test]
    fn ex2() {
        let nums = vec![5];
        let k = 1;

        let result = find_max_average(nums, k);

        assert_eq!(result, 5.0);
    }

    #[test]
    fn ex3() {
        let nums = vec![0, 1, 1, 3, 3];
        let k = 4;

        let result = find_max_average(nums, k);

        assert_eq!(result, 2.0);
    }
}
