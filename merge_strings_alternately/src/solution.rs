pub fn merge_alternately(word1: String, word2: String) -> String {
    let mut chars1 = word1.chars();
    let mut chars2 = word2.chars();

    let mut result: String = "".to_owned();

    loop {
        match (chars1.next(), chars2.next()) {
            (Some(a), Some(b)) => {
                result.push(a);
                result.push(b);
            }
            (Some(a), None) => result.push(a),
            (None, Some(b)) => result.push(b),
            (None, None) => break,
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex_one_same_length() {
        let result = merge_alternately("abc".into(), "pqr".into());

        assert_eq!(result, "apbqcr");
    }

    #[test]
    fn ex_two_second_is_longer() {
        let result = merge_alternately("ab".into(), "pqrs".into());

        assert_eq!(result, "apbqrs");
    }

    #[test]
    fn ex_three_first_is_longer() {
        let result = merge_alternately("abcd".into(), "pq".into());

        assert_eq!(result, "apbqcd");
    }
}
