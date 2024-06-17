pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    let max: i32 = *candies.iter().max().unwrap() - extra_candies;

    candies.iter().map(|&c| c >= max).collect()
}

#[cfg(test)]
mod test {
    use super::kids_with_candies;

    #[test]
    fn ex1() {
        let candies = vec![2, 3, 5, 1, 3];

        let result = kids_with_candies(candies, 3);

        assert_eq!(result, vec![true, true, true, false, true]);
    }

    #[test]
    fn ex2() {
        let candies = vec![4, 2, 1, 1, 2];

        let result = kids_with_candies(candies, 1);

        assert_eq!(result, vec![true, false, false, false, false]);
    }

    #[test]
    fn ex3() {
        let candies = vec![12, 1, 12];

        let result = kids_with_candies(candies, 10);

        assert_eq!(result, vec![true, false, true]);
    }
}
