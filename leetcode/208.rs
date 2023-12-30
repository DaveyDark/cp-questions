/*
Question:
A trie (pronounced as "try") or prefix tree is a tree data structure used to efficiently
store and retrieve keys in a dataset of strings.
There are various applications of this data structure, such as autocomplete and spellchecker.

Implement the Trie class:
    Trie() Initializes the trie object.
    void insert(String word) Inserts the string word into the trie.
    boolean search(String word) Returns true if the string word is in the trie (i.e., was inserted before), and false otherwise.
    boolean startsWith(String prefix) Returns true if there is a previously inserted string word that has the prefix prefix, and false otherwise.
*/

// Approach:
// 1) Create a Trie data structure with two fields: is_end and children.
//    is_end is a boolean that indicates if the current node is the end of a word.
//    children is a HashMap that maps a byte to a Trie node.
// 2) For insert, iterate through the word and insert each character into the Trie.
//    The last node of the word will have is_end set to true.
// 3) For search, iterate through the word and check if each character is in the Trie.
//    If you reach the end of the word, check if the is_end field is true.
// 4) For starts_with, iterate through the prefix and check if each character is in the Trie.
//    If you reach the end of the prefix, return true.

use std::collections::HashMap;

struct Trie {
    is_end: bool,
    children: HashMap<u8, Trie>,
}

impl Trie {
    fn new() -> Self {
        Trie {
            is_end: false,
            children: HashMap::new(),
        }
    }

    fn insert(&mut self, word: String) {
        let mut curr = self;
        for c in word.bytes() {
            curr = curr.children.entry(c).or_insert(Trie::new());
        }
        curr.is_end = true;
    }

    fn search(&self, word: String) -> bool {
        let mut curr = self;
        for c in word.bytes() {
            if let Some(c) = curr.children.get(&c) {
                curr = c;
            } else {
                return false;
            }
        }
        curr.is_end
    }

    fn starts_with(&self, prefix: String) -> bool {
        let mut curr = self;
        for c in prefix.bytes() {
            if let Some(c) = curr.children.get(&c) {
                curr = c;
            } else {
                return false;
            }
        }
        true
    }
}
