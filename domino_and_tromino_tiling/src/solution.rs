pub fn num_tilings(n: i32) -> i32 {
    match n {
        0 => return 0,
        1 => return 1,
        2 => return 2,
        3 => return 5,
        4 => return 11,
        _ => {}
    }

    let mut dp = Vec::new();
    dp.push(0);
    dp.push(1);
    dp.push(2);
    dp.push(5);
    dp.push(11);

    for i in 5..=n as usize {
        let result = dp[i - 1] as i64 * 2 + dp[i - 3] as i64;

        dp.push(result % 1_000_000_007);
    }

    dp[n as usize] as i32
}

#[cfg(test)]
mod tests {
    use super::num_tilings;

    #[test]
    fn ex1() {
        let result = num_tilings(3);

        assert_eq!(result, 5);
    }

    #[test]
    fn ex2() {
        let result = num_tilings(1);

        assert_eq!(result, 1);
    }

    #[test]
    fn ex3() {
        let result = num_tilings(4);

        assert_eq!(result, 11);
    }

    #[test]
    fn ex4() {
        let result = num_tilings(30);

        assert_eq!(result, 312342182);
    }
}
