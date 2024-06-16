use std::cmp;

pub fn largest_altitude(gain: Vec<i32>) -> i32 {
    let mut altitude = gain[0];
    let mut max = cmp::max(0, gain[0]);

    for i in 1..gain.len() {
        altitude += gain[i];

        if altitude > max {
            max = altitude
        }
    }

    max
}

#[cfg(test)]
mod tests {
    use super::largest_altitude;

    #[test]
    fn ex1() {
        let gain = vec![-5, 1, 5, 0, -7];

        let result = largest_altitude(gain);

        assert_eq!(result, 1);
    }

    #[test]
    fn ex2() {
        let gain = vec![-4, -3, -2, -1, 4, 3, 2];

        let result = largest_altitude(gain);

        assert_eq!(result, 0);
    }
}
