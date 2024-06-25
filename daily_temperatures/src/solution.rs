pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let mut results: Vec<i32> = vec![0; temperatures.len()];

    // monotoic stack with indices
    let mut stack = Vec::new();
    stack.push(0);

    for i in 1..temperatures.len() {
        let temperature = temperatures[i];

        while !stack.is_empty() && temperatures[*stack.last().unwrap()] < temperature {
            let top = stack.pop().unwrap();

            results[top] = (i - top) as i32;
        }

        stack.push(i);
    }

    results
}

#[cfg(test)]
mod tests {
    use super::daily_temperatures;

    #[test]
    fn ex1() {
        let temperatures = vec![73, 74, 75, 71, 69, 72, 76, 73];

        let result = daily_temperatures(temperatures);

        assert_eq!(result, vec![1, 1, 4, 2, 1, 1, 0, 0]);
    }

    #[test]
    fn ex2() {
        let temperatures = vec![30, 40, 50, 60];

        let result = daily_temperatures(temperatures);

        assert_eq!(result, vec![1, 1, 1, 0]);
    }

    #[test]
    fn ex3() {
        let temperatures = vec![30, 60, 90];

        let result = daily_temperatures(temperatures);

        assert_eq!(result, vec![1, 1, 0])
    }
}
