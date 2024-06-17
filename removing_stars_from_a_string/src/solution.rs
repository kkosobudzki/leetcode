pub fn remove_stars(s: String) -> String {
    let mut result = String::from("");

    for c in s.chars() {
        match c {
            '*' => {
                let _ = result.pop();
            }
            _ => result.push(c),
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::remove_stars;

    #[test]
    fn ex1() {
        let result = remove_stars(String::from("leet**cod*e"));

        assert_eq!(result, String::from("lecoe"));
    }

    #[test]
    fn ex2() {
        let result = remove_stars(String::from("erase*****"));

        assert_eq!(result, "")
    }
}
