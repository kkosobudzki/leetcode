fn gcd(a: usize, b: usize) -> usize {
    match (a, b) {
        (d, 0) => d,
        _ => gcd(b, a % b),
    }
}

pub fn gcd_of_strings(str1: String, str2: String) -> String {
    let gcd_length = gcd(str1.len(), str2.len());

    let divisor = &str1[0..gcd_length];

    if format!("{str1}{str2}") == format!("{str2}{str1}") {
        divisor.to_owned()
    } else {
        "".to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::gcd_of_strings;

    #[test]
    fn ex1_first_is_shorter() {
        let result = gcd_of_strings("ABCABC".into(), "ABC".into());

        assert_eq!(result, "ABC");
    }

    #[test]
    fn ex2() {
        let result = gcd_of_strings("ABABAB".into(), "ABAB".into());

        assert_eq!(result, "AB");
    }

    #[test]
    fn ex3_no_common_divisor() {
        let result = gcd_of_strings("LEET".into(), "CODE".into());

        assert_eq!(result, "");
    }

    #[test]
    fn ex4_no_common_divisor_junks_at_the_end() {
        let result = gcd_of_strings("ABCDEF".into(), "ABC".into());

        assert_eq!(result, "")
    }
}
