use std::cmp::max;

pub fn successful_pairs(spells: Vec<i32>, mut potions: Vec<i32>, success: i64) -> Vec<i32> {
    potions.sort_unstable();

    let mut result = vec![0; spells.len()];

    for i in 0..spells.len() {
        result[i] = binary_search(&potions, spells[i] as i64, success);
    }

    result
}

#[inline]
fn binary_search(potions: &[i32], spell: i64, success: i64) -> i32 {
    let length = potions.len();

    let mut low = 0;
    let mut high = length - 1;
    let mut result = 0;

    while high >= low {
        let mid = low + (high - low) / 2;

        if potions[mid] as i64 * spell >= success {
            result = max(result, length - mid);

            if mid == 0 {
                return result as i32;
            }

            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }

    result as i32
}

#[cfg(test)]
mod tests {
    use super::successful_pairs;

    #[test]
    fn ex1() {
        let result = successful_pairs(vec![5, 1, 3], vec![1, 2, 3, 4, 5], 7);

        assert_eq!(result, vec![4, 0, 3]);
    }

    #[test]
    fn ex2() {
        let result = successful_pairs(vec![3, 1, 2], vec![8, 5, 8], 16);

        assert_eq!(result, vec![2, 0, 2]);
    }

    #[test]
    fn ex3() {
        let result = successful_pairs(vec![15, 8, 19], vec![38, 36, 23], 328);

        assert_eq!(result, vec![3, 0, 3]);
    }

    #[test]
    fn ex4() {
        let result = successful_pairs(
            vec![
                15, 39, 38, 35, 33, 25, 31, 12, 40, 27, 29, 16, 22, 24, 7, 36, 29, 34, 24, 9, 11,
                35, 21, 3, 33, 10, 9, 27, 35, 17, 14, 3, 35, 35, 39, 23, 35, 14, 31, 7,
            ],
            vec![
                25, 19, 30, 37, 14, 30, 38, 22, 38, 38, 26, 33, 34, 23, 40, 28, 15, 29, 36, 39, 39,
                37, 32, 38, 8, 17, 39, 20, 4, 39, 39, 7, 30, 35, 29, 23,
            ],
            317,
        );

        assert_eq!(
            result,
            vec![
                28, 33, 33, 33, 33, 33, 33, 23, 34, 33, 33, 29, 32, 33, 0, 33, 33, 33, 33, 13, 22,
                33, 31, 0, 33, 17, 13, 33, 33, 30, 27, 0, 33, 33, 33, 33, 33, 27, 33, 0
            ]
        );
    }
}
