fn compress(chars: &mut Vec<char>) -> i32 {
    if chars.len() < 2 {
        return chars.len() as i32;
    }

    let mut last = chars[0];
    let mut last_count = 1;
    let mut end_of_the_array = 0;

    for i in 1..chars.len() {
        // println!("i: {i}, char: {:?}", chars[i]);

        match chars[i] {
            c if c == last => last_count += 1,
            other => {
                replace(chars, &mut end_of_the_array, last, last_count);

                last = other;
                last_count = 1;
            }
        }
    }

    replace(chars, &mut end_of_the_array, last, last_count);

    end_of_the_array as i32
}

#[inline]
fn replace(chars: &mut Vec<char>, end_of_the_array: &mut usize, c: char, count: u32) {
    chars[*end_of_the_array] = c;

    *end_of_the_array += 1;

    for digit in to_digits(count) {
        chars[*end_of_the_array] = digit;

        *end_of_the_array += 1;
    }
}

#[inline]
fn to_digits(num: u32) -> Vec<char> {
    match num {
        0 | 1 => vec![],
        2..=9 => vec![char::from_digit(num, 10).unwrap()],
        _ => format!("{num}").chars().collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::compress;

    #[test]
    fn ex1() {
        let mut chars = vec!['a', 'a', 'b', 'b', 'c', 'c', 'c'];

        let result = compress(&mut chars);

        assert_eq!(result, 6);
    }

    #[test]
    fn ex2() {
        let mut chars = vec!['a'];

        let result = compress(&mut chars);

        assert_eq!(result, 1);
    }

    #[test]
    fn ex3() {
        let mut chars = vec![
            'a', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b',
        ];

        let result = compress(&mut chars);

        assert_eq!(result, 4);
    }
}
