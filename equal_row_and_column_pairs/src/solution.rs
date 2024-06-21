pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();

    let mut columns = Vec::new();
    for i in 0..n {
        let mut column = vec![0; n];

        for j in 0..n {
            column[j] = grid[j][i];
        }

        columns.push(column);
    }

    let mut equals = 0;
    for row in grid.iter() {
        for column in columns.iter() {
            if row == column {
                equals += 1;
            }
        }
    }

    equals
}

#[cfg(test)]
mod tests {
    use super::equal_pairs;

    #[test]
    fn ex1() {
        let result = equal_pairs(vec![vec![3, 2, 1], vec![1, 7, 6], vec![2, 7, 7]]);

        assert_eq!(result, 1);
    }

    #[test]
    fn ex2() {
        let result = equal_pairs(vec![
            vec![3, 1, 2, 2],
            vec![1, 4, 4, 5],
            vec![2, 4, 2, 2],
            vec![2, 4, 2, 2],
        ]);

        assert_eq!(result, 3);
    }
}
