pub fn longest_subarray(nums: Vec<i32>) -> i32 {
    let mut index_of_zero: Option<usize> = None;
    let mut max = 0;

    let mut left = 0;
    let mut right = 0;

    while right < nums.len() {
        match nums[right] {
            0 if index_of_zero.is_none() => {
                index_of_zero = Some(right);

                right += 1;
            }
            0 if index_of_zero.is_some() => {
                let length = right - left - 1;

                if length > max {
                    max = length;
                }

                left = index_of_zero.unwrap() + 1;
                right = left;

                index_of_zero = None;
            }
            _ => {
                right += 1;
            }
        }
    }

    let last_length = right - left - 1;

    if last_length > max {
        last_length as i32
    } else {
        max as i32
    }
}

#[cfg(test)]
mod tests {
    use super::longest_subarray;

    #[test]
    fn ex1() {
        let result = longest_subarray(vec![1, 1, 0, 1]);

        assert_eq!(result, 3);
    }

    #[test]
    fn ex2() {
        let result = longest_subarray(vec![0, 1, 1, 1, 0, 1, 1, 0, 1]);

        assert_eq!(result, 5);
    }

    #[test]
    fn ex3() {
        let result = longest_subarray(vec![1, 1, 1]);

        assert_eq!(result, 2);
    }
}
