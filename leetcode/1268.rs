/*
Question:
You are given an array of strings products and a string searchWord.

Design a system that suggests at most three product names from products after each character of searchWord is typed.
Suggested products should have common prefix with searchWord.
If there are more than three products with a common prefix return the three lexicographically minimums products.

Return a list of lists of the suggested products after each character of searchWord is typed.
*/

// Approach:
// 1) Build a trie with all products
// 2) For each prefix, search the trie and return the first 3 results
// 3) Return the results
//
// TrieNode:
// 1) children: Vec<Option<TrieNode>>
// 2) is_end: bool
// 3) insert(ch): inserts a character into the trie
// 4) get(ch): get the child node of a character
// 5) get_mut(ch): get the mutable child node of a character
//
// Trie:
// 1) root: TrieNode
// 2) insert(word): insert a word into the Trie by inserting each character and setting the last character's is_end to true
// 3) search(prefix): search the Trie with a prefix by traversing the Trie via DFS and returning the first 3 results
// 4) dfs(curr, prefix, res): dfs to get the first 3 results

impl Solution {
    pub fn suggested_products(products: Vec<String>, search_word: String) -> Vec<Vec<String>> {
        #[derive(Clone)]
        struct TrieNode {
            children: Vec<Option<TrieNode>>,
            is_end: bool,
        }
        impl TrieNode {
            fn new() -> TrieNode {
                TrieNode {
                    children: vec![None; 26],
                    is_end: false,
                }
            }
            fn insert(&mut self, ch: u8) {
                self.children[(ch - b'a') as usize] = Some(TrieNode::new());
            }
            fn get(&self, ch: u8) -> Option<&TrieNode> {
                self.children[(ch - b'a') as usize].as_ref()
            }
            fn get_mut(&mut self, ch: u8) -> Option<&mut TrieNode> {
                self.children[(ch - b'a') as usize].as_mut()
            }
        }
        struct Trie {
            root: TrieNode,
        }
        impl Trie {
            fn new() -> Trie {
                Trie {
                    root: TrieNode::new(),
                }
            }
            fn insert(&mut self, word: String) {
                let mut curr = &mut self.root;
                for ch in word.bytes() {
                    if curr.get_mut(ch).is_none() {
                        curr.insert(ch);
                    }
                    curr = curr.get_mut(ch).unwrap();
                }
                curr.is_end = true;
            }
            fn search(&self, prefix: String) -> Vec<String> {
                let mut curr = &self.root;
                let mut res = vec![];
                for ch in prefix.bytes() {
                    if curr.get(ch).is_none() {
                        return res;
                    }
                    curr = curr.get(ch).unwrap();
                }
                self.dfs(curr, prefix, &mut res);
                res
            }
            fn dfs(&self, curr: &TrieNode, prefix: String, res: &mut Vec<String>) {
                if res.len() == 3 {
                    return;
                }
                if curr.is_end {
                    res.push(prefix.clone());
                }
                for ch in b'a'..=b'z' {
                    if let Some(n) = curr.get(ch) {
                        self.dfs(n, format!("{}{}", prefix, ch as char), res);
                    }
                }
            }
        }

        let mut trie = Trie::new();
        for product in products {
            trie.insert(product);
        }

        let mut ans = vec![];
        let mut prefix = String::new();
        for ch in search_word.chars() {
            prefix.push(ch);
            ans.push(trie.search(prefix.clone()));
        }
        ans
    }
}
