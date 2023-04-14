/*
Question:
Given a string s, find the first non-repeating character in it and return its index. If it does not exist, return -1.
*/

// Approach:
// 1) We use a hashmap and iterate over the string to record frequency of each character
// 2) We again iterate over the string and see if it's frequency in the map is 1, if it is then we return it
// 3) Otherwise we return -1 at the end of the function

use std::collections::HashMap;

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut map = HashMap::new();
        for ch in s.chars() {
            let val = map.entry(ch).or_insert(0);
            *val += 1;
        }
        for (i,ch) in s.char_indices() {
            if map[&ch] == 1 {return i as i32;}
        }
        -1
    }
}
