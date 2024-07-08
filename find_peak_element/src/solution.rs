pub fn find_peak_element(nums: Vec<i32>) -> i32 {
    let mut low = 0;
    let mut high = nums.len() - 1;

    while low < high {
        let mid = (low + high) / 2;

        if nums[mid] > nums[mid + 1] {
            high = mid;
        } else {
            low = mid + 1;
        }
    }

    low as i32
}

#[cfg(test)]
mod tests {
    use super::find_peak_element;

    #[test]
    fn ex1() {
        let result = find_peak_element(vec![1, 2, 3, 1]);

        assert_eq!(result, 2);
    }

    #[test]
    fn ex2() {
        let result = find_peak_element(vec![1, 2, 1, 3, 5, 6, 4]);

        assert_eq!(result, 5);
    }

    #[test]
    fn ex3() {
        let result = find_peak_element(vec![1]);

        assert_eq!(result, 0);
    }

    #[test]
    fn ex4() {
        let result = find_peak_element(vec![3, 2, 1]);

        assert_eq!(result, 0);
    }
}
