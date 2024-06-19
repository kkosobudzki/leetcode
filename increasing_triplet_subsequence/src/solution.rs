pub fn increasing_triplet(nums: Vec<i32>) -> bool {
    let length = nums.len();

    if length < 3 {
        return false;
    }

    let mut smallest = nums[0];

    let mut largest = vec![nums[length - 1]; length];
    let mut current_largest = nums[length - 1];

    for i in (0..length - 1).rev() {
        if nums[i] > current_largest {
            current_largest = nums[i];
        }

        largest[i] = current_largest;
    }

    for i in 1..length - 1 {
        if smallest < nums[i] && nums[i] < largest[i] {
            return true;
        }

        if smallest > nums[i] {
            smallest = nums[i];
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::increasing_triplet;

    #[test]
    fn ex1() {
        let result = increasing_triplet(vec![1, 2, 3, 4, 5]);

        assert_eq!(result, true);
    }

    #[test]
    fn ex2() {
        let result = increasing_triplet(vec![5, 4, 3, 2, 1]);

        assert_eq!(result, false);
    }

    #[test]
    fn ex3() {
        let result = increasing_triplet(vec![2, 1, 5, 0, 4, 6]);

        assert_eq!(result, true);
    }

    #[test]
    fn ex4() {
        let result = increasing_triplet(vec![20, 100, 10, 12, 5, 13]);

        assert_eq!(result, true);
    }

    #[test]
    fn ex5() {
        let result = increasing_triplet(vec![1, 5, 0, 4, 1, 3]);

        assert_eq!(result, true);
    }

    #[test]
    fn ex6() {
        let result = increasing_triplet(vec![6, 7, 1, 2]);

        assert_eq!(result, false);
    }

    #[test]
    fn ex7() {
        let result = increasing_triplet(vec![4, 5, 2147483647, 1, 2]);

        assert_eq!(result, true);
    }

    #[test]
    fn ex8() {
        let result = increasing_triplet(vec![1, 6, 2, 5, 1]);

        assert_eq!(result, true);
    }
}
