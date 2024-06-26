use std::collections::HashMap;

#[derive(Debug)]
struct Trie {
    children: HashMap<char, Trie>,
    is_terminal: bool,
}

impl Trie {
    fn new() -> Self {
        Trie {
            children: HashMap::new(),
            is_terminal: false,
        }
    }

    fn insert(&mut self, word: String) {
        let last_node = word.chars().fold(self, |node, c| {
            node.children.entry(c).or_insert(Trie::new())
        });

        last_node.is_terminal = true;
    }

    fn search(&self, word: String) -> bool {
        word.chars()
            .try_fold(self, |node, c| node.children.get(&c))
            .map_or(false, |node| node.is_terminal)
    }

    fn starts_with(&self, prefix: String) -> bool {
        prefix
            .chars()
            .try_fold(self, |node, c| node.children.get(&c))
            .is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::Trie;

    #[test]
    fn empty_trie() {
        let trie = Trie::new();

        assert_eq!(trie.search(String::from("krzychu")), false);
        assert_eq!(trie.search(String::from("dupa")), false);
        assert_eq!(trie.starts_with(String::from("krzy")), false);
        assert_eq!(trie.starts_with(String::from("d")), false);
    }

    #[test]
    fn contains_inserted_element() {
        let mut trie = Trie::new();
        trie.insert(String::from("krzychu"));

        assert_eq!(trie.search(String::from("krzychu")), true);
    }

    #[test]
    fn does_not_contain_non_inserted_element() {
        let mut trie = Trie::new();
        trie.insert(String::from("krzychu"));

        assert_eq!(trie.search(String::from("dupa")), false);
    }

    #[test]
    fn contains_prefix() {
        let mut trie = Trie::new();
        trie.insert(String::from("krzychu"));

        assert_eq!(trie.starts_with(String::from("krzy")), true);
    }

    #[test]
    fn does_not_contain_prefix() {
        let mut trie = Trie::new();
        trie.insert(String::from("krzychu"));

        assert_eq!(trie.starts_with(String::from("krzys")), false);
    }

    #[test]
    fn contains_element_that_is_prefix_of_another_word() {
        let mut trie = Trie::new();
        trie.insert(String::from("apple"));

        assert_eq!(trie.search(String::from("apple")), true);
        assert_eq!(trie.search(String::from("app")), false);
        assert_eq!(trie.starts_with(String::from("app")), true);

        trie.insert(String::from("app"));

        assert_eq!(trie.search(String::from("app")), true);
    }
}
