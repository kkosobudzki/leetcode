pub fn h_index(mut citations: Vec<i32>) -> i32 {
    citations.sort_unstable();

    let length = citations.len();

    let mut h = 0;

    for i in 0..length {
        h = h.max(citations[i].min((length - i) as i32))
    }

    h
}

#[cfg(test)]
mod tests {
    use super::h_index;

    #[test]
    fn ex1() {
        let result = h_index(vec![3, 0, 6, 1, 5]);

        assert_eq!(result, 3);
    }

    #[test]
    fn ex2() {
        let result = h_index(vec![1, 3, 1]);

        assert_eq!(result, 1);
    }

    #[test]
    fn ex3() {
        let result = h_index(vec![100]);

        assert_eq!(result, 1);
    }

    #[test]
    fn ex4() {
        let result = h_index(vec![11, 15]);

        assert_eq!(result, 2);
    }

    #[test]
    fn ex5() {
        let result = h_index(vec![4, 4, 0, 0]);

        assert_eq!(result, 2);
    }

    #[test]
    fn ex6() {
        let result = h_index(vec![9, 7, 6, 2, 1]);

        assert_eq!(result, 3);
    }
}
