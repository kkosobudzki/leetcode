use std::cmp::Ordering::{Equal, Greater, Less};

fn max_operations(mut nums: Vec<i32>, k: i32) -> i32 {
    nums.sort_unstable();

    let mut operations = 0;
    let mut left = 0;
    let mut right = nums.len() - 1;

    while left < right {
        match (nums[left] + nums[right]).cmp(&k) {
            Equal => {
                operations += 1;

                left += 1;
                right -= 1;
            }
            Less => left += 1,
            Greater => right -= 1,
        }
    }

    operations
}

#[cfg(test)]
mod tests {
    use super::max_operations;

    #[test]
    fn ex1() {
        let result = max_operations(vec![1, 2, 3, 4], 5);

        assert_eq!(result, 2);
    }

    #[test]
    fn ex2() {
        let result = max_operations(vec![3, 1, 3, 4, 3], 6);

        assert_eq!(result, 1);
    }

    #[test]
    fn ex3() {
        let result = max_operations(vec![2, 2, 2, 3, 1, 1, 4, 1], 4);

        assert_eq!(result, 2);
    }

    #[test]
    fn ex4() {
        let result = max_operations(
            vec![2, 5, 4, 4, 1, 3, 4, 4, 1, 4, 4, 1, 2, 1, 2, 2, 3, 2, 4, 2],
            3,
        );

        assert_eq!(result, 4);
    }
}
