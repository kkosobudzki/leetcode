pub fn roman_to_int(s: String) -> i32 {
    let mut sum = 0;
    let mut previous = 0;

    for c in s.chars().rev() {
        let value = match c {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,
        };

        if value < previous {
            sum -= value;
        } else {
            sum += value;
        }

        previous = value;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::roman_to_int;

    #[test]
    fn ex1() {
        let result = roman_to_int(String::from("III"));

        assert_eq!(result, 3);
    }

    #[test]
    fn ex2() {
        let result = roman_to_int(String::from("LVIII"));

        assert_eq!(result, 58);
    }

    #[test]
    fn ex3() {
        let result = roman_to_int(String::from("MCMXCIV"));

        assert_eq!(result, 1994);
    }
}
