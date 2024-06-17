pub fn is_subsequence(s: String, t: String) -> bool {
    if t.len() < s.len() {
        return false;
    }

    let mut t_chars: Vec<char> = t.chars().collect();

    s.chars().all(|c| {
        for i in 0..t_chars.len() {
            if t_chars[i] == c {
                t_chars.drain(..i + 1);

                return true;
            }
        }

        false
    })
}

#[cfg(test)]
mod tests {
    use super::is_subsequence;

    #[test]
    fn ex1() {
        let result = is_subsequence(String::from("abc"), String::from("ahbgdc"));

        assert_eq!(result, true);
    }

    #[test]
    fn ex2() {
        let result = is_subsequence(String::from("axc"), String::from("ahbgdc"));

        assert_eq!(result, false);
    }
}
