pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
    match points.len() {
        0 => return 0,
        1 => return 1,
        _ => points.sort_unstable(),
    }

    let mut arrows = 1;

    let mut right = points[0][1];

    for i in 1..points.len() {
        if right < points[i][0] {
            arrows += 1;

            right = points[i][1];
        } else {
            right = right.min(points[i][1]);
        }
    }

    arrows
}

#[cfg(test)]
mod tests {
    use super::find_min_arrow_shots;

    #[test]
    fn ex1() {
        let points = vec![vec![10, 16], vec![2, 8], vec![1, 6], vec![7, 12]];

        let result = find_min_arrow_shots(points);

        assert_eq!(result, 2);
    }

    #[test]
    fn ex2() {
        let points = vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8]];

        let result = find_min_arrow_shots(points);

        assert_eq!(result, 4);
    }

    #[test]
    fn ex3() {
        let points = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]];

        let result = find_min_arrow_shots(points);

        assert_eq!(result, 2);
    }
}
