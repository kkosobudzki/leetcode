fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
    let length = is_connected.len();

    let mut parent = Vec::from_iter(0..length);
    let mut size = vec![1; length];

    let mut count = length as i32;

    for i in 0..length {
        for j in i + 1..length {
            if is_connected[i][j] == 1 {
                count -= union(i, j, &mut parent, &mut size);
            }
        }
    }

    count
}

#[inline]
fn union(first_node: usize, second_node: usize, parent: &mut [usize], size: &mut [i32]) -> i32 {
    let first_root = find(first_node, parent);
    let second_root = find(second_node, parent);

    if first_root == second_root {
        return 0;
    }

    if size[first_root] > size[second_root] {
        parent[second_root] = first_root;
        size[first_root] += 1;
    } else {
        parent[first_root] = second_root;
        size[second_root] += 1;
    }

    1
}

#[inline]
fn find(node: usize, parent: &mut [usize]) -> usize {
    let mut n = node;

    while n != parent[n] {
        parent[n] = parent[parent[n]];

        n = parent[n];
    }

    n
}

#[cfg(test)]
mod tests {
    use super::find_circle_num;

    #[test]
    fn ex1() {
        let is_connected = vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1]];

        let result = find_circle_num(is_connected);

        assert_eq!(result, 2);
    }

    #[test]
    fn ex2() {
        let is_connected = vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]];

        let result = find_circle_num(is_connected);

        assert_eq!(result, 3);
    }
}
