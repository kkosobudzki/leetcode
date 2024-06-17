use std::collections::VecDeque;

pub fn predict_party_victory(senate: String) -> String {
    let mut senators = VecDeque::from_iter(senate.chars());
    let mut to_remove: VecDeque<char> = VecDeque::new();

    while senators.len() > 1 {
        let senator = senators.pop_front().unwrap(); // it cannot be empty

        match to_remove.pop_front() {
            Some(s) if s == senator => {} // removing
            Some(s) if s != senator => {
                if senators.iter().all(|&in_game| in_game == senator) {
                    return senators_party(senator);
                }

                to_remove.push_front(s); // return to the queue

                to_remove.push_back(s); // removing an opposite of s
                senators.push_back(senator); // stays in the game
            }
            _ => {
                to_remove.push_back(opposite_of(senator)); // removing an opposite of s
                senators.push_back(senator); // stays in the game
            }
        }
    }

    senators_party(senators.pop_front().unwrap())
}

#[inline]
fn senators_party(senator: char) -> String {
    match senator {
        'R' => String::from("Radiant"),
        _ => String::from("Dire"),
    }
}

#[inline]
fn opposite_of(senator: char) -> char {
    match senator {
        'R' => 'D',
        _ => 'R',
    }
}

#[cfg(test)]
mod tests {
    use super::predict_party_victory;

    #[test]
    fn ex1_only_one_who_can_vote() {
        let result = predict_party_victory(String::from("RD"));

        assert_eq!(result, "Radiant");
    }

    #[test]
    fn ex2() {
        let result = predict_party_victory(String::from("RDD"));

        assert_eq!(result, "Dire");
    }

    #[test]
    fn ex3() {
        let result = predict_party_victory(String::from("DDRRRR"));

        assert_eq!(result, "Radiant");
    }
}
