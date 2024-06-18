use std::collections::VecDeque;

pub fn predict_party_victory(senate: String) -> String {
    let mut senators = VecDeque::from_iter(senate.chars());
    let mut to_remove: VecDeque<char> = VecDeque::new();

    while senators.len() > 1 {
        match (to_remove.front(), senators.pop_front()) {
            (Some(&ban), Some(senator)) if ban == senator => {
                // banning first matched senator
                to_remove.pop_front();
            }
            (_, Some(senator)) => {
                // this sucks
                if senators.iter().all(|&s| s == senator) {
                    // all senators from the same party - win
                    return senators_party(senator);
                } else {
                    // removing opposite of senator
                    to_remove.push_back(opposite_of(senator));
                    // senator's going to the next round
                    senators.push_back(senator);
                }
            }
            _ => {
                panic!(
                    "uncovered case, to_remove: {:?}, senators: {:?}",
                    to_remove, senators
                )
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
