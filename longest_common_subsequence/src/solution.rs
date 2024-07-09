use std::cmp::max;

pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
    let len1 = text1.len();
    let len2 = text2.len();

    let mut dp = vec![vec![0; len2 + 1]; len1 + 1];

    let bytes1 = text1.as_bytes();
    let bytes2 = text2.as_bytes();

    for i in 0..len1 {
        for j in 0..len2 {
            if bytes1[i] == bytes2[j] {
                dp[i + 1][j + 1] = dp[i][j] + 1
            } else {
                dp[i + 1][j + 1] = max(dp[i][j + 1], dp[i + 1][j])
            }
        }
    }

    dp[len1][len2]
}

#[cfg(test)]
mod tests {
    use super::longest_common_subsequence;

    #[test]
    fn ex1() {
        let result = longest_common_subsequence(String::from("abcde"), String::from("ace"));

        assert_eq!(result, 3);
    }

    #[test]
    fn ex2() {
        let result = longest_common_subsequence(String::from("abc"), String::from("abc"));

        assert_eq!(result, 3);
    }

    #[test]
    fn ex3() {
        let result = longest_common_subsequence(String::from("abc"), String::from("def"));

        assert_eq!(result, 0);
    }
}
