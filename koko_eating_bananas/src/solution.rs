pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
    let max = *piles.iter().max().unwrap();

    let mut low = 1;
    let mut high = max;

    while low < high {
        let k = (low + high) / 2;

        let time: i64 = piles
            .iter()
            .map(|&p| (p as f64 / k as f64).ceil() as i64)
            .sum();

        if time as i32 <= h {
            high = k;
        } else {
            low = k + 1;
        }
    }

    low
}

#[cfg(test)]
mod tests {
    use super::min_eating_speed;

    #[test]
    fn ex1() {
        let result = min_eating_speed(vec![3, 6, 7, 11], 8);

        assert_eq!(result, 4);
    }

    #[test]
    fn ex2() {
        let result = min_eating_speed(vec![30, 11, 23, 4, 20], 5);

        assert_eq!(result, 30);
    }

    #[test]
    fn ex3() {
        let result = min_eating_speed(vec![30, 11, 23, 4, 20], 6);

        assert_eq!(result, 23);
    }

    #[test]
    fn ex4() {
        let result = min_eating_speed(vec![312884470], 312884469);

        assert_eq!(result, 2);
    }

    #[test]
    fn ex5() {
        let result = min_eating_speed(vec![1, 1, 1, 999999999], 10);

        assert_eq!(result, 142857143);
    }

    #[test]
    fn ex6() {
        let result = min_eating_speed(vec![805306368, 805306368, 805306368], 1000000000);

        assert_eq!(result, 3);
    }
}
