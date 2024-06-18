pub fn reverse_vowels(s: String) -> String {
    let mut left = 0;
    let mut right = s.len() - 1;

    let mut bytes = s.into_bytes();

    while left < right {
        match (is_vowel(bytes[left]), is_vowel(bytes[right])) {
            (true, true) => {
                bytes.swap(left, right);

                left += 1;
                right -= 1;
            }
            (false, true) | (false, false) => left += 1,
            (true, false) => right -= 1,
        }
    }

    String::from_utf8(bytes).unwrap()
}

#[inline]
fn is_vowel(b: u8) -> bool {
    match b {
        b'a' | b'A' | b'e' | b'E' | b'i' | b'I' | b'o' | b'O' | b'u' | b'U' => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::reverse_vowels;

    #[test]
    fn ex1() {
        let result = reverse_vowels(String::from("hello"));

        assert_eq!(result, String::from("holle"));
    }

    #[test]
    fn ex2() {
        let result = reverse_vowels(String::from("leetcode"));

        assert_eq!(result, String::from("leotcede"));
    }

    #[test]
    fn ex3_with_upper_case() {
        let result = reverse_vowels(String::from("Aa"));

        assert_eq!(result, String::from("aA"));
    }
}
