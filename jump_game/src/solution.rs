pub fn can_jump(nums: Vec<i32>) -> bool {
    match nums.len() {
        0 => return false,
        1 => return true,
        _ => {}
    }

    let mut i = nums.len() - 2;
    let mut length = 1;

    while i > 0 {
        if nums[i] >= length {
            length = 1;
        } else {
            length += 1;
        }

        i -= 1;
    }

    nums[0] >= length
}

#[cfg(test)]
mod tests {
    use super::can_jump;

    #[test]
    fn ex1() {
        let result = can_jump(vec![2, 3, 1, 1, 4]);

        assert_eq!(result, true);
    }

    #[test]
    fn ex2() {
        let result = can_jump(vec![3, 2, 1, 0, 4]);

        assert_eq!(result, false);
    }

    #[test]
    fn ex3() {
        let result = can_jump(vec![]);

        assert_eq!(result, false);
    }

    #[test]
    fn ex4() {
        let result = can_jump(vec![0]);

        assert_eq!(result, true);
    }
}
