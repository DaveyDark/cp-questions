/* 
Question:
Given two strings s and t, return true if t is an anagram of s, and false otherwise.
An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase, 
typically using all the original letters exactly once.
*/

// Approach:
// 1) We calculate frequency of all characters with a hashmap in the first string
// 2) We subtract 1 for each ocurring character in the second string
// 3) We go through the hashmap and if any frequency != 0 then the words aren't anagrams
// 4) Otherwise we return true at the end of the function

use std::collections::HashMap;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut map = HashMap::new();
        for ch in s.chars() {
            let val = map.entry(ch).or_insert(0);
            *val += 1;
        }
        for ch in t.chars() {
            let val = map.entry(ch).or_insert(0);
            *val += -1;
        }
        for (key,val) in map {
            if val != 0 {return false;}
        }
        true
    }
}
