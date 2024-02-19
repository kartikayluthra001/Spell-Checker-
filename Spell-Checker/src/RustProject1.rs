/*
the project is to create an application where we will be implementing a
command line tool for spell checking in rust
 */

use std::collections::HashMap;

#[derive(Default)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_end_of_word: bool,
}

struct Trie {
    root: TrieNode,
}

impl Trie {
    fn new() -> Self {
        Trie { root: TrieNode::default() }
    }

    fn insert(&mut self, word: &str) {
        let mut current = &mut self.root;
        for c in word.chars() {
            current = current.children.entry(c).or_insert(TrieNode::default());
        }
        current.is_end_of_word = true;
    }

    fn search(&self, word: &str) -> bool {
        let mut current = &self.root;
        for c in word.chars() {
            if let Some(node) = current.children.get(&c) {
                current = node;
            } else {
                return false;
            }
        }
        current.is_end_of_word
    }

    fn suggest_corrections(&self, word: &str) -> Vec<String> {
        let mut suggestions = Vec::new();
        let mut current = &self.root;

        // Bit manipulation to generate corrections
        for i in 0..=word.len() {
            let prefix = &word[..i];
            let suffix = &word[i..];

            // Skip empty suffix
            if suffix.is_empty() {
                continue;
            }

            // Check for deletions
            let deletion_candidate = format!("{}{}", prefix, &suffix[1..]);
            if self.search(&deletion_candidate) {
                suggestions.push(deletion_candidate);
            }

            // Check for substitutions
            for c in 'a'..'z' {
                let substitution_candidate = format!("{}{}{}", prefix, c, &suffix[1..]);
                if self.search(&substitution_candidate) {
                    suggestions.push(substitution_candidate);
                }
            }
        }

        suggestions
    }
}

fn main() {
    // Example usage
    let mut trie = Trie::new();

    // Insert words into the trie (build the dictionary)
    trie.insert("apple");
    trie.insert("banana");
    trie.insert("orange");

    // Spell check and suggest corrections
    let word_to_check = "aplpe";
    if !trie.search(word_to_check) {
        println!("The word '{}' is misspelled.", word_to_check);

        // Suggest corrections
        let corrections = trie.suggest_corrections(word_to_check);
        if corrections.is_empty() {
            println!("No suggestions found.");
        } else {
            println!("Suggestions:");
            for suggestion in corrections {
                println!("{}", suggestion);
            }
        }
    } else {
        println!("The word '{}' is correctly spelled.", word_to_check);
    }
}
