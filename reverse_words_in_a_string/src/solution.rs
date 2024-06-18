pub fn reverse_words(s: String) -> String {
    let mut result = s
        .split_whitespace()
        .rev()
        .fold(String::from(""), |mut result, word| {
            result.push_str(word);
            result.push_str(" ");

            result
        });

    result.pop();

    result
}

#[cfg(test)]
mod tests {
    use super::reverse_words;

    #[test]
    fn ex1() {
        let result = reverse_words(String::from("the sky is blue"));

        assert_eq!(result, String::from("blue is sky the"));
    }

    #[test]
    fn ex2() {
        let result = reverse_words(String::from("  hello world  "));

        assert_eq!(result, String::from("world hello"));
    }

    #[test]
    fn ex3() {
        let result = reverse_words(String::from("a good   example"));

        assert_eq!(result, String::from("example good a"));
    }
}
