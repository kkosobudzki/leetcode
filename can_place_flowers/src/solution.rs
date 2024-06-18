pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
    if n == 0 {
        return true;
    }

    let mut planted = flowerbed.clone();

    let length = planted.len();

    let mut free = 0;

    for i in 0..length {
        if planted[i] == 0
            && (i == 0 || planted[i - 1] == 0)
            && (i == length - 1 || planted[i + 1] == 0)
        {
            free += 1;

            planted[i] = 1;

            if free >= n {
                return true;
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::can_place_flowers;

    #[test]
    fn ex1() {
        let result = can_place_flowers(vec![1, 0, 0, 0, 1], 1);

        assert_eq!(result, true);
    }

    #[test]
    fn ex2() {
        let result = can_place_flowers(vec![1, 0, 0, 0, 1], 2);

        assert_eq!(result, false);
    }

    #[test]
    fn ex3() {
        let result = can_place_flowers(vec![1, 0, 0, 0, 0, 1], 2);

        assert_eq!(result, false);
    }

    #[test]
    fn ex4() {
        let result = can_place_flowers(vec![1, 0, 1, 0, 1, 0, 1], 0);

        assert_eq!(result, true);
    }
}
