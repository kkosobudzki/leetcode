pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut products = vec![1; nums.len()];

    let mut prefix = 1;
    let mut sufix = 1;

    let length = nums.len();

    for i in 0..length {
        products[i] *= prefix;
        products[length - i - 1] *= sufix;

        prefix *= nums[i];
        sufix *= nums[length - i - 1];
    }

    products
}

#[cfg(test)]
mod tests {
    use super::product_except_self;

    #[test]
    fn ex1() {
        let result = product_except_self(vec![1, 2, 3, 4]);

        assert_eq!(result, vec![24, 12, 8, 6]);
    }

    #[test]
    fn ex2() {
        let result = product_except_self(vec![-1, 1, 0, -3, 3]);

        assert_eq!(result, vec![0, 0, 9, 0, 0]);
    }
}
