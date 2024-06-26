use std::i32;

struct StockSpanner {
    stack: Vec<(i32, i32)>,
}

impl StockSpanner {
    fn new() -> Self {
        StockSpanner { stack: Vec::new() }
    }

    fn next(&mut self, price: i32) -> i32 {
        let mut count = 1;

        while let Some((top_price, top_count)) = self.stack.last() {
            if *top_price > price {
                break;
            }

            count += top_count;

            self.stack.pop();
        }

        self.stack.push((price, count));

        count
    }
}

#[cfg(test)]
mod test {
    use super::StockSpanner;

    #[test]
    fn ex1() {
        let mut spanner = StockSpanner::new();

        assert_eq!(spanner.next(100), 1);
        assert_eq!(spanner.next(80), 1);
        assert_eq!(spanner.next(60), 1);
        assert_eq!(spanner.next(70), 2);
        assert_eq!(spanner.next(60), 1);
        assert_eq!(spanner.next(75), 4);
        assert_eq!(spanner.next(85), 6);
    }
}
