use std::collections::VecDeque;

pub fn nearest_exit(maze: Vec<Vec<char>>, entrance: Vec<i32>) -> i32 {
    let e = (entrance[0] as usize, entrance[1] as usize, 0);

    let mut queue = VecDeque::new();
    queue.push_back(e);

    let height = maze.len();
    let width = maze[0].len();

    let mut visited: Vec<Vec<bool>> = vec![vec![false; width]; height];
    visited[e.0][e.1] = true;

    while let Some(point) = queue.pop_front() {
        for m in moves(point.0, point.1, &maze) {
            if visited[m.0][m.1] == false {
                if is_exit(m, width, height) {
                    return point.2 + 1;
                }

                visited[m.0][m.1] = true;

                queue.push_back((m.0, m.1, point.2 + 1));
            }
        }
    }

    -1
}

#[inline]
fn is_exit(point: (usize, usize), width: usize, height: usize) -> bool {
    point.0 == 0 || point.0 == height - 1 || point.1 == 0 || point.1 == width - 1
}

#[inline]
fn moves(x: usize, y: usize, maze: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let height = maze.len();
    let width = maze[0].len();

    let mut moves = Vec::new();

    if x > 0 && maze[x - 1][y] == '.' {
        // left
        moves.push((x - 1, y));
    }

    if x < height - 1 && maze[x + 1][y] == '.' {
        // right
        moves.push((x + 1, y));
    }

    // up
    if y > 0 && maze[x][y - 1] == '.' {
        moves.push((x, y - 1));
    }

    // down
    if y < width - 1 && maze[x][y + 1] == '.' {
        moves.push((x, y + 1));
    }

    moves
}

#[cfg(test)]
mod tests {
    use super::nearest_exit;

    #[test]
    fn ex1() {
        let maze = vec![
            vec!['+', '+', '.', '+'],
            vec!['.', '.', '.', '+'],
            vec!['+', '+', '+', '.'],
        ];

        let result = nearest_exit(maze, vec![1, 2]);

        assert_eq!(result, 1);
    }

    #[test]
    fn ex2() {
        let maze = vec![
            vec!['+', '+', '+'],
            vec!['.', '.', '.'],
            vec!['+', '+', '+'],
        ];

        let result = nearest_exit(maze, vec![1, 0]);

        assert_eq!(result, 2);
    }

    #[test]
    fn ex3() {
        let maze = vec![vec!['.', '+']];

        let result = nearest_exit(maze, vec![0, 0]);

        assert_eq!(result, -1);
    }

    #[test]
    fn ex4() {
        let maze = vec![
            vec!['+', '.', '+', '+', '+', '+', '+'],
            vec!['+', '.', '+', '.', '.', '.', '+'],
            vec!['+', '.', '+', '.', '+', '.', '+'],
            vec!['+', '.', '.', '.', '.', '.', '+'],
            vec!['+', '+', '+', '+', '.', '+', '.'],
        ];

        let result = nearest_exit(maze, vec![0, 1]);

        assert_eq!(result, 7);
    }
}
