/*
Question:
Given two strings ransomNote and magazine, return true if ransomNote can be constructed by using the letters from magazine and false otherwise.
Each letter in magazine can only be used once in ransomNote.
*/

// Approach:
// 1) We use a map and calculate frequency of characters in magazine
// 2) Then we go over ransom_note and subtract 1 each ocurring character
// 3) If any character has < 0 frequency then it isn't present in magazine enough times so we return false
// 3) Otherwise we return -1 at the end of the function

use std::collections::HashMap;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut map = HashMap::new();

        for ch in magazine.chars() {
            let val = map.entry(ch).or_insert(0);
            *val += 1;
        }
        for ch in ransom_note.chars() {
            let val = map.entry(ch).or_insert(0);
            *val -= 1;
        }
        for (key,val) in map {
            if val < 0 {return false;}
        }
        true
    }
}
