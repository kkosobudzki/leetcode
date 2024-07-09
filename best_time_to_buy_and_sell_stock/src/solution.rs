pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
    let n = prices.len();

    if n <= 1 {
        return 0;
    }

    let mut buy = -prices[0];
    let mut sell = 0;

    for i in 1..n {
        buy = buy.max(sell - prices[i]);
        sell = sell.max(prices[i] - fee + buy);
    }

    sell
}

#[cfg(test)]
mod tests {
    use super::max_profit;

    #[test]
    fn ex1() {
        let result = max_profit(vec![1, 3, 2, 8, 4, 9], 2);

        assert_eq!(result, 8);
    }

    #[test]
    fn ex2() {
        let result = max_profit(vec![1, 3, 7, 5, 10, 3], 3);

        assert_eq!(result, 6);
    }
}
