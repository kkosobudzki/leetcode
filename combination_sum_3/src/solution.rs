pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
    let mut combinations: Vec<Vec<i32>> = Vec::new();

    combination(&mut vec![], 1, k, n, &mut combinations);

    combinations
}

fn combination(current: &mut Vec<i32>, i: i32, k: i32, n: i32, combinations: &mut Vec<Vec<i32>>) {
    if k == 0 {
        if is_correct(current, n) {
            combinations.push(current.clone());
        }

        return;
    }

    if i >= n || i > 9 {
        return;
    }

    current.push(i);

    // firstly expand up to k numbers
    combination(current, i + 1, k - 1, n, combinations);

    current.pop();

    // secondly try different numbers
    combination(current, i + 1, k, n, combinations);
}

#[inline]
fn is_correct(current: &[i32], n: i32) -> bool {
    current.iter().fold(0, |sum, &num| sum + num) == n
}

#[cfg(test)]
mod tests {
    use super::combination_sum3;

    #[test]
    fn ex1() {
        let result = combination_sum3(3, 7);

        assert_eq!(result, vec![vec![1, 2, 4]]);
    }

    #[test]
    fn ex2() {
        let result = combination_sum3(3, 9);

        assert_eq!(result, vec![vec![1, 2, 6], vec![1, 3, 5], vec![2, 3, 4]]);
    }

    #[test]
    fn ex3() {
        let result = combination_sum3(4, 1);

        let expected: Vec<Vec<i32>> = vec![];

        assert_eq!(result, expected);
    }

    #[test]
    fn ex4() {
        let result = combination_sum3(9, 45);

        assert_eq!(result, vec![vec![1, 2, 3, 4, 5, 6, 7, 8, 9]]);
    }
}
