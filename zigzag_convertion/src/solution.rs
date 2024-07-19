pub fn convert(s: String, num_rows: i32) -> String {
    let nu = num_rows as usize;

    let mut rows = vec![String::new(); nu];
    let mut i = 0;
    let mut down = true;

    for c in s.chars() {
        rows[i].push(c);

        if down {
            if i == nu - 1 {
                down = false;

                if i > 0 {
                    i -= 1;
                }
            } else if i < nu - 1 {
                i += 1;
            }
        } else {
            if i == 0 {
                down = true;

                if i < nu - 1 {
                    i += 1;
                }
            } else if i > 0 {
                i -= 1;
            }
        }
    }

    rows.join("")
}

#[cfg(test)]
mod tests {
    use super::convert;

    #[test]
    fn ex1() {
        let result = convert(String::from("PAYPALISHIRING"), 3);

        assert_eq!(result, String::from("PAHNAPLSIIGYIR"));
    }

    #[test]
    fn ex2() {
        let result = convert(String::from("PAYPALISHIRING"), 4);

        assert_eq!(result, String::from("PINALSIGYAHRPI"));
    }

    #[test]
    fn ex3() {
        let result = convert(String::from("A"), 1);

        assert_eq!(result, String::from("A"));
    }

    #[test]
    fn ex4() {
        let result = convert(String::from("ABC"), 1);

        assert_eq!(result, String::from("ABC"));
    }
}
