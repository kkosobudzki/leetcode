pub fn tribonacci(n: i32) -> i32 {
    if n < 3 {
        return match n {
            0 => 0,
            1 | 2 => 1,
            _ => -1,
        };
    }

    let nu = n as usize;

    let mut f: Vec<i32> = vec![0; nu + 1];
    f[0] = 0;
    f[1] = 1;
    f[2] = 1;

    for i in 3..nu + 1 {
        f[i] = f[i - 1] + f[i - 2] + f[i - 3];
    }

    f[nu]
}

#[cfg(test)]
mod tests {
    use super::tribonacci;

    #[test]
    fn ex1() {
        let result = tribonacci(4);

        assert_eq!(result, 4);
    }

    #[test]
    fn ex2() {
        let result = tribonacci(25);

        assert_eq!(result, 1389537);
    }
}
