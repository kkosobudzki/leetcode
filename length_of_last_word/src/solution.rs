pub fn length_of_last_word(s: String) -> i32 {
    let mut length = 0;

    for char in s.chars().rev() {
        if char.is_whitespace() {
            if length > 0 {
                return length;
            }
        } else {
            length += 1
        }
    }

    length
}

#[cfg(test)]
mod tests {
    use super::length_of_last_word;

    #[test]
    fn ex1() {
        let result = length_of_last_word(String::from("Hello World"));

        assert_eq!(result, 5);
    }

    #[test]
    fn ex2() {
        let result = length_of_last_word(String::from("   fly me   to   the moon  "));

        assert_eq!(result, 4);
    }

    #[test]
    fn ex3() {
        let result = length_of_last_word(String::from("luffy is still joyboy"));

        assert_eq!(result, 6);
    }

    #[test]
    fn ex4() {
        let result = length_of_last_word(String::from("a"));

        assert_eq!(result, 1);
    }
}
