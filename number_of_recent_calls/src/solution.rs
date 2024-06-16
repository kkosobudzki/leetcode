use std::collections::VecDeque;

struct RecentCounter {
    timeline: VecDeque<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RecentCounter {
    fn new() -> Self {
        Self {
            timeline: VecDeque::new(),
        }
    }

    fn ping(&mut self, t: i32) -> i32 {
        loop {
            match self.timeline.pop_back() {
                Some(time) if time >= t - 3_000 => {
                    self.timeline.push_back(time);

                    break;
                }
                Some(_) => {} // removing old elements
                None => break,
            }
        }

        self.timeline.push_front(t);

        self.timeline.len() as i32
    }
}

/**
 * Your RecentCounter object will be instantiated and called as such:
 * let obj = RecentCounter::new();
 * let ret_1: i32 = obj.ping(t);
 */

#[cfg(test)]
mod tests {
    use super::RecentCounter;

    #[test]
    fn ex1() {
        let mut counter = RecentCounter::new();

        assert_eq!(counter.ping(1), 1);
        assert_eq!(counter.ping(100), 2);
        assert_eq!(counter.ping(3001), 3);
        assert_eq!(counter.ping(3002), 3);
    }

    #[test]
    fn ex2() {
        let mut counter = RecentCounter::new();

        assert_eq!(counter.ping(1178), 1);
        assert_eq!(counter.ping(1483), 2);
        assert_eq!(counter.ping(1563), 3);
        assert_eq!(counter.ping(4054), 4);
        assert_eq!(counter.ping(4152), 5);
    }
}
