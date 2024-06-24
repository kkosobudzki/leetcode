use std::collections::HashMap;

pub fn letter_combinations(digits: String) -> Vec<String> {
    let mut map = HashMap::new();
    map.insert('2', vec!['a', 'b', 'c']);
    map.insert('3', vec!['d', 'e', 'f']);
    map.insert('4', vec!['g', 'h', 'i']);
    map.insert('5', vec!['j', 'k', 'l']);
    map.insert('6', vec!['m', 'n', 'o']);
    map.insert('7', vec!['p', 'q', 'r', 's']);
    map.insert('8', vec!['t', 'u', 'v']);
    map.insert('9', vec!['w', 'x', 'y', 'z']);

    let mut combinations = Vec::new();

    combination(digits.chars().collect(), 0, &map, &mut combinations);

    combinations
}

fn combination(
    digits: Vec<char>,
    index: usize,
    map: &HashMap<char, Vec<char>>,
    combinations: &mut Vec<String>,
) {
    if index == digits.len() {
        return;
    }

    let num = digits[index];

    if let Some(options) = map.get(&num) {
        for &letter in options {
            let mut d = digits.clone();
            d[index] = letter;

            if is_correct(&d) {
                let combination = String::from_iter(d.clone());

                combinations.push(combination);
            }

            combination(d, index + 1, map, combinations);
        }
    }
}

fn is_correct(digits: &Vec<char>) -> bool {
    digits.iter().all(|&d| match d {
        'a'..='z' => true,
        _ => false,
    })
}

#[cfg(test)]
mod tests {
    use super::letter_combinations;

    #[test]
    fn ex1() {
        let result = letter_combinations(String::from("23"));

        assert_eq!(
            result,
            vec![
                String::from("ad"),
                String::from("ae"),
                String::from("af"),
                String::from("bd"),
                String::from("be"),
                String::from("bf"),
                String::from("cd"),
                String::from("ce"),
                String::from("cf")
            ]
        )
    }

    #[test]
    fn ex2() {
        let result = letter_combinations(String::from(""));

        let expected: Vec<String> = vec![];

        assert_eq!(result, expected);
    }

    #[test]
    fn ex3() {
        let result = letter_combinations(String::from("2"));

        assert_eq!(result, vec!["a", "b", "c"]);
    }
}
