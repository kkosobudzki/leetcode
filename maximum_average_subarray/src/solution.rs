pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
    let k64 = f64::from(k);
    let ku = k as usize;

    let mut sum = nums[0..ku]
        .iter()
        .fold(0.0, |acc, &num| acc + f64::from(num));
    let mut max = sum / k64;

    for i in 1..nums.len() - ku + 1 {
        sum = sum - f64::from(nums[i - 1]) + f64::from(nums[i + ku - 1]);

        let average = sum / k64;

        if average > max {
            max = average;
        }
    }

    max
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
