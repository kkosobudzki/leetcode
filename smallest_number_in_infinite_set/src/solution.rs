use std::collections::VecDeque;

struct SmallestInfiniteSet {
    numbers: VecDeque<i32>,
}

impl SmallestInfiniteSet {
    fn new() -> Self {
        let mut set = SmallestInfiniteSet {
            numbers: VecDeque::from_iter(1..=1_000),
        };

        for i in (0..499).rev() {
            set.heapify(i);
        }

        set
    }

    fn pop_smallest(&mut self) -> i32 {
        if self.numbers.len() == 1 {
            return self.numbers.pop_front().unwrap();
        }

        let smallest = self.numbers[0];

        self.numbers[0] = self.numbers.pop_back().unwrap();

        self.heapify(0);

        smallest
    }

    fn add_back(&mut self, num: i32) {
        if self.numbers.iter().find(|&&n| n == num).is_some() {
            return;
        }

        self.numbers.push_back(num);

        let mut current = self.numbers.len() - 1;
        let mut parent = (current - 1) / 2;

        while current > 0 && self.numbers[parent] > self.numbers[current] {
            self.numbers.swap(current, parent);

            current = parent;

            if current > 0 {
                parent = (current - 1) / 2;
            }
        }
    }

    fn heapify(&mut self, i: usize) {
        let size = self.numbers.len();

        let mut smallest = i;

        let left_index = 2 * i + 1;
        if size > left_index && self.numbers[left_index] < self.numbers[smallest] {
            smallest = left_index;
        }

        let right_index = left_index + 1;
        if size > right_index && self.numbers[right_index] < self.numbers[smallest] {
            smallest = right_index;
        }

        if smallest != i {
            self.numbers.swap(smallest, i);
            self.heapify(smallest);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SmallestInfiniteSet;

    #[test]
    fn ex1() {
        let mut obj = SmallestInfiniteSet::new();
        obj.add_back(2);

        assert_eq!(obj.pop_smallest(), 1);
        assert_eq!(obj.pop_smallest(), 2);
        assert_eq!(obj.pop_smallest(), 3);

        obj.add_back(1);

        assert_eq!(obj.pop_smallest(), 1);
        assert_eq!(obj.pop_smallest(), 4);
        assert_eq!(obj.pop_smallest(), 5);
    }
}
