use std::cmp::min;

pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
    let mut dp = vec![0; cost.len() + 1];

    let last_index = cost.len() - 1;

    dp[last_index] = cost[last_index];

    for i in (0..last_index).rev() {
        dp[i] = min(dp[i + 1], dp[i + 2]) + cost[i];
    }

    min(dp[0], dp[1])
}

#[cfg(test)]
mod tests {
    use super::min_cost_climbing_stairs;

    #[test]
    fn ex1() {
        let result = min_cost_climbing_stairs(vec![10, 15, 20]);

        assert_eq!(result, 15);
    }

    #[test]
    fn ex2() {
        let result = min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1]);

        assert_eq!(result, 6);
    }
}
