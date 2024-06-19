pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
    let mut stack: Vec<i32> = Vec::new();
    let mut negative_winners: Vec<i32> = Vec::new();

    for &asteroid in asteroids.iter() {
        if asteroid >= 0 {
            stack.push(asteroid);
        } else {
            loop {
                match stack.pop() {
                    Some(moving_right) => {
                        match moving_right + asteroid {
                            0 => break, // both explodes
                            1.. => {
                                stack.push(moving_right);

                                break;
                            }
                            _ => {} // moving left wins => keep trying to destroys these moving to
                                    // the right
                        }
                    }
                    None => {
                        // nothing more to destroy
                        negative_winners.push(asteroid);

                        break;
                    }
                }
            }
        }
    }

    if negative_winners.is_empty() {
        stack
    } else {
        negative_winners.append(&mut stack);

        negative_winners
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::asteroid_collision;

    #[test]
    fn ex1() {
        let result = asteroid_collision(vec![5, 10, -5]);

        assert_eq!(result, vec![5, 10]);
    }

    #[test]
    fn ex2() {
        let result = asteroid_collision(vec![8, -8]);

        assert_eq!(result, vec![]);
    }

    #[test]
    fn ex3() {
        let result = asteroid_collision(vec![10, 2, -5]);

        assert_eq!(result, vec![10]);
    }

    #[test]
    fn ex4() {
        let result = asteroid_collision(vec![-2, -1, 1, 2]);

        assert_eq!(result, vec![-2, -1, 1, 2]);
    }
}
