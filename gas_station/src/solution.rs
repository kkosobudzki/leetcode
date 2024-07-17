pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    assert!(gas.len() == cost.len());

    let mut total = 0;
    let mut sum_loop = 0;
    let mut start = 0;

    for i in 0..gas.len() {
        total += gas[i] - cost[i];
        sum_loop += gas[i] - cost[i];

        if sum_loop < 0 {
            // starting here failed, try next
            start = (i + 1) % gas.len();
            sum_loop = 0;
        }
    }

    if total >= 0 {
        return start as i32;
    }

    -1
}

#[cfg(test)]
mod tests {
    use super::can_complete_circuit;

    #[test]
    fn ex1() {
        let result = can_complete_circuit(vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2]);

        assert_eq!(result, 3);
    }

    #[test]
    fn ex2() {
        let result = can_complete_circuit(vec![2, 3, 4], vec![3, 4, 3]);

        assert_eq!(result, -1);
    }
}
