pub fn min_distance(word1: String, word2: String) -> i32 {
    let m = word1.len();
    let n = word2.len();

    let bytes1 = word1.as_bytes();
    let bytes2 = word2.as_bytes();

    let mut distance = vec![vec![0; n + 1]; m + 1];

    for i in 1..=m {
        distance[i][0] = i;
    }

    for j in 1..=n {
        distance[0][j] = j;
    }

    for i in 1..=m {
        for j in 1..=n {
            if bytes1[i - 1] == bytes2[j - 1] {
                distance[i][j] = distance[i - 1][j - 1];
            } else {
                distance[i][j] = (distance[i - 1][j - 1])
                    .min(distance[i - 1][j])
                    .min(distance[i][j - 1])
                    + 1;
            }
        }
    }

    distance[m][n] as i32
}

#[cfg(test)]
mod tests {
    use super::min_distance;

    #[test]
    fn ex1() {
        let result = min_distance(String::from("horse"), String::from("ros"));

        assert_eq!(result, 3);
    }

    #[test]
    fn ex2() {
        let result = min_distance(String::from("intention"), String::from("execution"));

        assert_eq!(result, 5);
    }
}
