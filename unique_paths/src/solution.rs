pub fn unique_paths(m: i32, n: i32) -> i32 {
    let nu = n as usize;
    let mu = m as usize;

    let mut pascal = vec![vec![0; nu]; mu];

    for i in 0..mu {
        pascal[i][0] = 1;
    }

    for j in 0..nu {
        pascal[0][j] = 1;
    }

    for i in 1..mu {
        for j in 1..nu {
            pascal[i][j] = pascal[i - 1][j] + pascal[i][j - 1];
        }
    }

    pascal[mu - 1][nu - 1]
}

#[cfg(test)]
mod tests {
    use super::unique_paths;

    #[test]
    fn ex1() {
        let result = unique_paths(3, 7);

        assert_eq!(result, 28);
    }

    #[test]
    fn ex2() {
        let result = unique_paths(3, 2);

        assert_eq!(result, 3)
    }
}
