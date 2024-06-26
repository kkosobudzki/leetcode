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

    fn suggest(&self, prefix: &str) -> Vec<String> {
        prefix
            .chars()
            .try_fold(self, |node, c| node.children.get(&c))
            .map_or(vec![], |node| {
                let mut suggestions: Vec<String> = Vec::new();

                node.expand(prefix.to_string(), &mut suggestions);

                suggestions.sort_unstable();
                suggestions.truncate(3);

                suggestions
            })
    }

    fn expand(&self, prefix: String, words: &mut Vec<String>) {
        if self.is_terminal {
            words.push(prefix.to_string());
        }

        self.children
            .iter()
            .for_each(|(c, node)| node.expand(format!("{prefix}{c}"), words))
    }
}

pub fn suggested_products(products: Vec<String>, search_word: String) -> Vec<Vec<String>> {
    let mut trie = Trie::new();
    products
        .iter()
        .for_each(|product| trie.insert(product.clone()));

    let mut results: Vec<Vec<String>> = Vec::new();

    for i in 1..=search_word.len() {
        results.push(trie.suggest(&search_word[..i]))
    }

    results
}

#[cfg(test)]
mod tests {
    use super::suggested_products;

    #[test]
    fn ex1() {
        let products = vec![
            String::from("mobile"),
            String::from("mouse"),
            String::from("moneypot"),
            String::from("monitor"),
            String::from("mousepad"),
        ];
        let expected = vec![
            vec!["mobile", "moneypot", "monitor"],
            vec!["mobile", "moneypot", "monitor"],
            vec!["mouse", "mousepad"],
            vec!["mouse", "mousepad"],
            vec!["mouse", "mousepad"],
        ];

        let result = suggested_products(products, String::from("mouse"));

        assert_eq!(result, expected);
    }

    #[test]
    fn ex2() {
        let products = vec![String::from("havana")];
        let expected = vec![
            vec!["havana"],
            vec!["havana"],
            vec!["havana"],
            vec!["havana"],
            vec!["havana"],
            vec!["havana"],
        ];

        let result = suggested_products(products, String::from("havana"));

        assert_eq!(result, expected);
    }
}
