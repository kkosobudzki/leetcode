pub fn single_number(nums: Vec<i32>) -> i32 {
    // a ^ b = 1
    // a ^ a = 0
    // a ^ b ^ a = 1
    // a ^ b ^ a ^ b = 0

    let mut missing = nums[0];

    for i in 1..nums.len() {
        missing = missing ^ nums[i];
    }

    missing
}

#[cfg(test)]
mod tests {
    use super::single_number;

    #[test]
    fn ex1() {
        let result = single_number(vec![2, 2, 1]);

        assert_eq!(result, 1);
    }

    #[test]
    fn ex2() {
        let result = single_number(vec![4, 1, 2, 1, 2]);

        assert_eq!(result, 4);
    }

    #[test]
    fn ex3() {
        let result = single_number(vec![1]);

        assert_eq!(result, 1);
    }
}
