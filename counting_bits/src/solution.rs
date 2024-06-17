pub fn count_bits(n: i32) -> Vec<i32> {
    if n < 2 {
        return match n {
            0 => vec![0],
            _ => vec![0, 1],
        };
    }

    let nu = n as usize;

    let mut ans: Vec<i32> = vec![0; nu + 1];
    ans[0] = 0;
    ans[1] = 1;

    for i in 2..nu + 1 {
        ans[i] = ans[i / 2] + i as i32 % 2; // even = 0 at the end, odd = 1 at the end
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::count_bits;

    #[test]
    fn ex1() {
        let result = count_bits(2);

        assert_eq!(result, vec![0, 1, 1]);
    }

    #[test]
    fn ex2() {
        let result = count_bits(5);

        assert_eq!(result, vec![0, 1, 1, 2, 1, 2]);
    }
}
