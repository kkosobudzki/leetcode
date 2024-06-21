use core::panic;
use std::collections::VecDeque;

pub fn decode_string(s: String) -> String {
    let mut stack = VecDeque::new();

    for c in s.chars() {
        match c {
            ']' => {
                let content = parse_single_content(&mut stack);

                for cc in content.chars() {
                    stack.push_back(cc);
                }
            }
            _ => stack.push_back(c),
        }
    }

    let mut result = String::new();
    for c in stack {
        result.push(c);
    }

    result
}

#[inline]
fn parse_single_content(stack: &mut VecDeque<char>) -> String {
    let mut content = String::new();

    while let Some(c) = stack.pop_back() {
        match c {
            '[' => {
                let times = parse_times(stack);
                
                return repeat(&content, times);

            }
            'a'..='z' | 'A'..='Z' => content.insert(0, c),
            _ => panic!("unexpected char while parsing single content"),
        }
    }

    content
}

#[inline]
fn parse_times(stack: &mut VecDeque<char>) -> usize {
    let mut times = String::new();

    while let Some(c) = stack.pop_back() {
        match c {
            '0'..='9' => times.insert(0, c),
            _ => {
                // push back something i do not handle right now
                stack.push_back(c);

                break;
            }
        
        }
    }

    if let Ok(count) = times.parse() {
        return count;
    }

    0
}

#[inline]
fn repeat(s: &str, times: usize) -> String {
    let mut result = String::new();

    for _ in 0..times {
        result.push_str(s);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::decode_string;

    #[test]
    fn ex1() {
        let result = decode_string(String::from("3[a]2[bc]"));

        assert_eq!(result, String::from("aaabcbc"));
    }

    #[test]
    fn ex2() {
        let result = decode_string(String::from("3[a2[c]]"));

        assert_eq!(result, String::from("accaccacc"));
    }

    #[test]
    fn ex3() {
        let result = decode_string(String::from("2[abc]3[cd]ef"));

        assert_eq!(result, String::from("abcabccdcdcdef"));
    }

    #[test]
    fn ex4() {
        let result = decode_string(String::from("abc3[cd]xyz"));

        assert_eq!(result, String::from("abccdcdcdxyz"));
    }

    #[test]
    fn ex5() {
        let result = decode_string(String::from("3[z]2[2[y]pq4[2[jk]e1[f]]]ef"));

        assert_eq!(
            result,
            String::from("zzzyypqjkjkefjkjkefjkjkefjkjkefyypqjkjkefjkjkefjkjkefjkjkefef")
        );
    }
}
