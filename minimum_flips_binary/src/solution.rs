use std::cmp::max;

pub fn min_flips(a: i32, b: i32, c: i32) -> i32 {
    let mut flips = 0;

    let mut a_chars: Vec<char> = format!("{a:b}").chars().collect();
    let mut b_chars: Vec<char> = format!("{b:b}").chars().collect();
    let mut chars: Vec<char> = format!("{c:b}").chars().collect();

    let length = max(max(a_chars.len(), b_chars.len()), chars.len());

    // leftpad - npm can you see it? xD
    while a_chars.len() < length {
        a_chars.insert(0, '0');
    }

    while b_chars.len() < length {
        b_chars.insert(0, '0');
    }

    while chars.len() < length {
        chars.insert(0, '0');
    }

    for i in 0..length {
        match chars[i] {
            '0' => match (a_chars[i], b_chars[i]) {
                ('0', '1') | ('1', '0') => flips += 1,
                ('1', '1') => flips += 2,
                _ => {}
            },
            '1' => match (a_chars[i], b_chars[i]) {
                ('0', '0') => flips += 1,
                _ => {}
            },
            _ => unreachable!(),
        }
    }

    flips
}

#[cfg(test)]
mod tests {
    use super::min_flips;

    #[test]
    fn ex1() {
        let result = min_flips(2, 6, 5);

        assert_eq!(result, 3);
    }

    #[test]
    fn ex2() {
        let result = min_flips(4, 2, 7);

        assert_eq!(result, 1);
    }

    #[test]
    fn ex3() {
        let result = min_flips(1, 2, 3);

        assert_eq!(result, 0);
    }

    #[test]
    fn ex4() {
        let result = min_flips(8, 3, 5);

        assert_eq!(result, 3);
    }
}
