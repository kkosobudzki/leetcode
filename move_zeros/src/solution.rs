pub fn move_zeroes(nums: &mut Vec<i32>) {
    let length = nums.len();

    let mut shift = 0;

    for i in 0..length {
        if i + shift < length {
            while nums[i + shift] == 0 && i + shift + 1 < length {
                shift += 1;
            }

            nums[i] = nums[i + shift];
        } else {
            nums[i] = 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let mut nums = vec![0, 1, 0, 3, 12];

        move_zeroes(&mut nums);

        assert_eq!(nums, vec![1, 3, 12, 0, 0]);
    }

    #[test]
    fn ex2() {
        let mut nums = vec![0];

        move_zeroes(&mut nums);

        assert_eq!(nums, vec![0]);
    }

    #[test]
    fn ex3() {
        let mut nums = vec![0, 0, 1];

        move_zeroes(&mut nums);

        assert_eq!(nums, vec![1, 0, 0]);
    }
}
