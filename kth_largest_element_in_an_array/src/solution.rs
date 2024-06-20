use std::collections::VecDeque;

pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
    let ku = k as usize;

    let mut heap = VecDeque::with_capacity(ku);

    for num in nums {
        if heap.len() < ku {
            heap.push_back(num);
        } else if num > heap[0] {
            heap.pop_front();
            heap.push_back(num);
        }

        let i = heap.len() - 1;

        heapify(&mut heap, i);
    }

    heap[0]
}

fn heapify(arr: &mut VecDeque<i32>, i: usize) {
    let parent = parent_index(i);
    let mut smallest = parent;

    let left = 2 * parent + 1;

    if arr.len() > left && arr[left] < arr[smallest] {
        smallest = left;
    }

    let right = left + 1;

    if arr.len() > right && arr[right] < arr[smallest] {
        smallest = right;
    }

    arr.swap(smallest, parent);

    if parent > 0 {
        heapify(arr, parent);
    }
}

fn parent_index(i: usize) -> usize {
    ((i as f64 - 1.0) / 2.0).floor() as usize
}

#[cfg(test)]
mod tests {
    use super::find_kth_largest;

    #[test]
    fn ex1() {
        let result = find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2);

        assert_eq!(result, 5);
    }

    #[test]
    fn ex2() {
        let result = find_kth_largest(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4);

        assert_eq!(result, 4);
    }

    #[test]
    fn ex3() {
        let result = find_kth_largest(vec![1, 3, 3, 2, 2, 4, 5, 5, 6], 4);

        assert_eq!(result, 4);
    }

    #[test]
    fn ex4() {
        let result = find_kth_largest(vec![-1, 2, 0], 2);

        assert_eq!(result, 0);
    }

    #[test]
    fn ex5() {
        let result = find_kth_largest(vec![2, 1], 1);

        assert_eq!(result, 2);
    }

    #[test]
    fn ex6() {
        let result = find_kth_largest(vec![99, 99], 1);

        assert_eq!(result, 99);
    }

    #[test]
    fn ex7() {
        let result = find_kth_largest(
            vec![
                3, 2, 3, 1, 2, 4, 5, 5, 6, 7, 7, 8, 2, 3, 1, 1, 1, 10, 11, 5, 6, 2, 4, 7, 8, 5, 6,
            ],
            20,
        );

        assert_eq!(result, 2);
    }
}
