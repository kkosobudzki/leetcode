use std::collections::VecDeque;

pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
    let mut queue = VecDeque::new();

    let x_len = grid.len();
    let y_len = grid[0].len();

    for x in 0..x_len {
        for y in 0..y_len {
            if grid[x][y] == 2 {
                queue.push_back((x, y, 0));
            }
        }
    }

    let mut minutes = 0;

    while let Some(head) = queue.pop_front() {
        let (x, y, m) = head;

        if grid[x][y] == 1 {
            // only update when actually rotten
            minutes = m;
        }

        grid[x][y] = 2;

        if x > 0 && grid[x - 1][y] == 1 {
            queue.push_back((x - 1, y, m + 1));
        }

        if x + 1 < x_len && grid[x + 1][y] == 1 {
            queue.push_back((x + 1, y, m + 1))
        }

        if y > 0 && grid[x][y - 1] == 1 {
            queue.push_back((x, y - 1, m + 1));
        }

        if y + 1 < y_len && grid[x][y + 1] == 1 {
            queue.push_back((x, y + 1, m + 1));
        }
    }

    for x in 0..x_len {
        for y in 0..y_len {
            if grid[x][y] == 1 {
                return -1;
            }
        }
    }

    minutes
}

#[cfg(test)]
mod tests {
    use super::oranges_rotting;

    #[test]
    fn ex1() {
        let grid = vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1]];

        let result = oranges_rotting(grid);

        assert_eq!(result, 4);
    }

    #[test]
    fn ex2() {
        let grid = vec![vec![2, 1, 1], vec![0, 1, 1], vec![1, 0, 1]];

        let result = oranges_rotting(grid);

        assert_eq!(result, -1);
    }

    #[test]
    fn ex3() {
        let result = oranges_rotting(vec![vec![0, 2]]);

        assert_eq!(result, 0);
    }

    #[test]
    fn ex4() {
        let result = oranges_rotting(vec![vec![2, 2], vec![1, 1], vec![0, 0], vec![2, 0]]);

        assert_eq!(result, 1);
    }
}
