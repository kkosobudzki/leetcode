pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let mut k = nums.len();

    let mut left = 0;
    let mut right = nums.len() - 1;

    while left <= right {
        if nums[left] == val {
            if nums[right] != val {
                nums.swap(left, right);

                left += 1;
            }

            if right == 0 {
                return k as i32 - 1;
            }

            right -= 1;
            k -= 1;
        } else {
            left += 1;
        }
    }

    k as i32
}

#[cfg(test)]
mod tests {
    use super::remove_element;

    #[test]
    fn ex1() {
        let mut nums = vec![3, 2, 2, 3];

        let result = remove_element(&mut nums, 3);

        assert_eq!(result, 2);
    }

    #[test]
    fn ex2() {
        let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];

        let result = remove_element(&mut nums, 2);

        assert_eq!(result, 5);
    }

    #[test]
    fn ex3() {
        let mut nums = vec![];

        let result = remove_element(&mut nums, 0);

        assert_eq!(result, 0);
    }

    #[test]
    fn ex4() {
        let mut nums = vec![1];

        let result = remove_element(&mut nums, 1);

        assert_eq!(result, 0);
    }

    #[test]
    fn ex5() {
        let mut nums = vec![4, 5];

        let result = remove_element(&mut nums, 4);

        assert_eq!(result, 1);
    }
}
