use std::cmp;

pub fn max_area(height: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = height.len() - 1;

    let mut hl = height[left];
    let mut hr = height[right];
    let mut width = right - left;
    let mut max = 0;

    while left < right {
        let area = width * (cmp::min(hl, hr) as usize);

        if area > max {
            max = area;
        }

        if hl < hr {
            left += 1;

            hl = height[left];
        } else {
            right -= 1;

            hr = height[right];
        }

        width -= 1;
    }

    max as i32
}

#[cfg(test)]
mod tests {
    use super::max_area;

    #[test]
    fn ex1() {
        let result = max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]);

        assert_eq!(result, 49);
    }

    #[test]
    fn ex2() {
        let result = max_area(vec![1, 1]);

        assert_eq!(result, 1);
    }

    #[test]
    fn ex3() {
        let result = max_area(vec![2, 3, 4, 5, 18, 17, 6]);

        assert_eq!(result, 17);
    }
}
