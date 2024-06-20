pub fn close_strings(word1: String, word2: String) -> bool {
    if word1.len() != word2.len() {
        return false;
    }

    let mut oc1 = vec![0; 26];
    let mut oc2 = vec![0; 26];

    for &b in word1.as_bytes() {
        oc1[(b - b'a') as usize] += 1;
    }

    for &b in word2.as_bytes() {
        oc2[(b - b'a') as usize] += 1;
    }

    // has to verify whether both contain the same letters
    for i in 0..26 {
        match (oc1[i], oc2[i]) {
            (0, 1..) => return false,
            (1.., 0) => return false,
            _ => {} // 0 & 0 && different non zero values are fine
        }
    }

    oc1.sort_unstable();
    oc2.sort_unstable();

    oc1 == oc2
}

#[cfg(test)]
mod tests {
    use super::close_strings;

    #[test]
    fn ex1() {
        let result = close_strings(String::from("abc"), String::from("bca"));

        assert_eq!(result, true);
    }

    #[test]
    fn ex2() {
        let result = close_strings(String::from("a"), String::from("aa"));

        assert_eq!(result, false);
    }

    #[test]
    fn ex3() {
        let result = close_strings(String::from("cabbba"), String::from("abbccc"));

        assert_eq!(result, true);
    }

    #[test]
    fn ex4() {
        let result = close_strings(String::from("abbzzca"), String::from("babzzcz"));

        assert_eq!(result, false);
    }

    #[test]
    fn ex5() {
        let result = close_strings(String::from("uau"), String::from("ssx"));

        assert_eq!(result, false);
    }
}
