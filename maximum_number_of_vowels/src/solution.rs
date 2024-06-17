pub fn max_vowels(s: String, k: i32) -> i32 {
    // vowels a, e, i, o, u

    let ku = k as usize;

    let chars: Vec<char> = s.chars().collect();

    let mut max = 0;

    for i in 0..ku {
        if is_vowel(chars[i]) {
            max += 1;
        }
    }

    let mut sum = max;

    for i in 1..s.len() - ku + 1 {
        if is_vowel(chars[i - 1]) {
            sum -= 1;
        }

        if is_vowel(chars[i + ku - 1]) {
            sum += 1;
        }

        if sum > max {
            max = sum;
        }
    }

    max
}

#[inline]
fn is_vowel(c: char) -> bool {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::max_vowels;

    #[test]
    fn ex1() {
        let result = max_vowels(String::from("abciiidef"), 3);

        assert_eq!(result, 3)
    }

    #[test]
    fn ex2() {
        let result = max_vowels(String::from("aeiou"), 2);

        assert_eq!(result, 2)
    }

    #[test]
    fn ex3() {
        let result = max_vowels(String::from("leetcode"), 3);

        assert_eq!(result, 2)
    }

    #[test]
    fn ex4() {
        let result = max_vowels(String::from("weallloveyou"), 7);

        assert_eq!(result, 4);
    }
}
