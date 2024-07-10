use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn total_cost(costs: Vec<i32>, mut k: i32, candidates: i32) -> i64 {
    let cu = candidates as usize;

    let mut left: BinaryHeap<Reverse<i64>> = BinaryHeap::with_capacity(cu);
    let mut right: BinaryHeap<Reverse<i64>> = BinaryHeap::with_capacity(cu);

    let mut left_cursor = cu;
    let mut right_cursor = costs.len() - cu - 1;

    costs[..left_cursor]
        .iter()
        .for_each(|&c| left.push(Reverse(c as i64)));

    costs[left_cursor.max(right_cursor + 1)..]
        .iter()
        .for_each(|&c| right.push(Reverse(c as i64)));

    let mut total = 0i64;

    let max = &Reverse(std::i64::MAX);

    while k > 0 {
        k -= 1;

        let a: i64 = left.peek().unwrap_or(max).0;
        let b: i64 = right.peek().unwrap_or(max).0;

        if a <= b {
            total += a;

            left.pop();

            if left_cursor <= right_cursor && left_cursor < costs.len() {
                left.push(Reverse(costs[left_cursor] as i64));

                left_cursor += 1;
            }
        } else {
            total += b;

            right.pop();

            if left_cursor <= right_cursor {
                right.push(Reverse(costs[right_cursor] as i64));

                right_cursor -= 1;
            }
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::total_cost;

    #[test]
    fn ex1() {
        let result = total_cost(vec![17, 12, 10, 2, 7, 2, 11, 20, 8], 3, 4);

        assert_eq!(result, 11);
    }

    #[test]
    fn ex2() {
        let result = total_cost(vec![1, 2, 4, 1], 3, 3);

        assert_eq!(result, 4);
    }

    #[test]
    fn ex3() {
        let result = total_cost(
            vec![
                18, 64, 12, 21, 21, 78, 36, 58, 88, 58, 99, 26, 92, 91, 53, 10, 24, 25, 20, 92, 73,
                63, 51, 65, 87, 6, 17, 32, 14, 42, 46, 65, 43, 9, 75,
            ],
            13,
            23,
        );

        assert_eq!(result, 223);
    }

    #[test]
    fn ex4() {
        let result = total_cost(
            vec![
                28, 35, 21, 13, 21, 72, 35, 52, 74, 92, 25, 65, 77, 1, 73, 32, 43, 68, 8, 100, 84,
                80, 14, 88, 42, 53, 98, 69, 64, 40, 60, 23, 99, 83, 5, 21, 76, 34,
            ],
            32,
            12,
        );

        assert_eq!(result, 1407);
    }
}
