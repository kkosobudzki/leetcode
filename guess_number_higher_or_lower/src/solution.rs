static mut PICKED: i32 = 0;

unsafe fn guess(n: i32) -> i32 {
    match n {
        a if a == PICKED => 0,
        a if a > PICKED => -1,
        _ => 1,
    }
}

pub unsafe fn guess_number(n: i32) -> i32 {
    let mut left = 1;
    let mut right = n;

    while left <= right {
        let mid = ((left as f64 + right as f64) / 2.0).floor() as i32;

        match guess(mid) {
            0 => return mid,
            1 => {
                left = mid + 1;
            }
            _ => {
                right = mid - 1;
            }
        }
    }

    -1
}

#[cfg(test)]
mod tests {
    use super::{guess_number, PICKED};

    #[test]
    fn ex1() {
        unsafe {
            PICKED = 6;

            let result = guess_number(10);

            assert_eq!(result, PICKED);
        }
    }

    #[test]
    fn ex2() {
        unsafe {
            PICKED = 1;

            let result = guess_number(1);

            assert_eq!(result, PICKED);
        }
    }

    #[test]
    fn ex3() {
        unsafe {
            PICKED = 1;

            let result = guess_number(2);

            assert_eq!(result, PICKED);
        }
    }

    #[test]
    fn ex4() {
        unsafe {
            PICKED = 1702766719;

            let result = guess_number(2126753390);

            assert_eq!(result, PICKED);
        }
    }
}
