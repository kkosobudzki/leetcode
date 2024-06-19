use std::cmp;

pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
    let mut max = 0;

    let mut left = 0;
    let mut right = 0;

    let mut zeros_left = k;

    while right < nums.len() {
        match nums[right] {
            0 if zeros_left > 0 => {
                zeros_left -= 1;
                right += 1;
            }
            1 => right += 1,
            _ => {
                if right - left > max {
                    max = right - left;
                }

                zeros_left = k;
                left += 1;
                right = left;
            }
        }
    }

    cmp::max(max, right - left) as i32
}

#[cfg(test)]
mod tests {
    use super::longest_ones;

    #[test]
    fn ex1() {
        let result = longest_ones(vec![1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0], 2);

        assert_eq!(result, 6);
    }

    #[test]
    fn ex2() {
        let result = longest_ones(
            vec![0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1],
            3,
        );

        assert_eq!(result, 10);
    }

    #[test]
    fn ex3() {
        let result = longest_ones(vec![0, 0, 0, 1], 4);

        assert_eq!(result, 4);
    }
}
