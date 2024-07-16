use std::fs::File;
use std::io::Read;

struct RandomizedSet {
    array: Vec<i32>,
    random: File,
}

impl RandomizedSet {
    fn new() -> Self {
        Self {
            array: Vec::new(),
            random: File::open("/dev/random").expect("Cannot open /dev/random"),
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        match self.array.iter().position(|&x| x == val) {
            Some(_) => false,
            None => {
                self.array.push(val);

                true
            }
        }
    }

    fn remove(&mut self, val: i32) -> bool {
        match self.array.iter().position(|&x| x == val) {
            Some(index) => {
                self.array.remove(index);

                true
            }
            None => false,
        }
    }

    fn get_random(&mut self) -> i32 {
        let mut buffer = vec![0u8; 1];
        self.random
            .read_exact(&mut buffer)
            .expect("Cannot read random value");

        let index = buffer[0] as usize % self.array.len();

        self.array[index]
    }
}

#[cfg(test)]
mod tests {
    use super::RandomizedSet;

    #[test]
    fn ex1() {
        let mut obj = RandomizedSet::new();

        assert_eq!(true, obj.insert(1));
        assert_eq!(false, obj.remove(2));
        assert_eq!(true, obj.insert(2));

        obj.get_random();

        assert_eq!(true, obj.remove(1));
        assert_eq!(false, obj.insert(2));

        obj.get_random();
    }
}
