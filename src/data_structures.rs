use std::collections::HashMap;

#[derive(Debug)]
pub struct Trie {
    children: HashMap<char, Trie>,
    leaf: Option<String>
}

impl Trie {
    pub fn new() -> Trie {
        Self {
            children: HashMap::new(),
            leaf: None,
        }
    }

    pub fn insert(&mut self, remaining: &str, word: &str) {
        let mut chars = remaining.chars();
        
        let first = chars.next();

        if let Some(character) = first {
            let child = self.children.entry(character).or_insert(Trie::new());

            child.insert(&chars.collect::<String>(), word);
        }
        else {
            self.leaf = Some(word.to_owned())
        }
    }

    pub fn search(&self, word: &str) -> Option<String> {
        let mut chars = word.chars();

        if let Some(character) = chars.next() {
            if let Some(child) = self.children.get(&character) {
                child.search(&chars.collect::<String>())
            }
            else {
                self.leaf.clone()
            }
        }
        else {
            self.leaf.clone()
        }
    }
}