pub fn int_to_roman(mut num: i32) -> String {
    let symbols_with_values = vec![
        (1_000, "M"),
        (900, "CM"),
        (500, "D"),
        (400, "CD"),
        (100, "C"),
        (90, "XC"),
        (50, "L"),
        (40, "XL"),
        (10, "X"),
        (9, "IX"),
        (5, "V"),
        (4, "IV"),
        (1, "I"),
    ];

    let mut result = String::new();

    for (value, symbol) in symbols_with_values {
        while num >= value {
            result.push_str(symbol);

            num -= value;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::int_to_roman;

    #[test]
    fn ex1() {
        let result = int_to_roman(3749);

        assert_eq!(result, String::from("MMMDCCXLIX"));
    }

    #[test]
    fn ex2() {
        let result = int_to_roman(58);

        assert_eq!(result, String::from("LVIII"));
    }

    #[test]
    fn ex3() {
        let result = int_to_roman(1994);

        assert_eq!(result, String::from("MCMXCIV"));
    }
}
