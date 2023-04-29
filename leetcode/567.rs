/*
Question:
Given two strings s1 and s2, return true if s2 contains a permutation of s1, or false otherwise.
In other words, return true if one of s1's permutations is the substring of s2.
*/

// Approach:
// 1) First we calculate frequency of chars in the first string and store it in a hashmap
// 2) Then we make a second hashmap to keep track of the characters in the current sliding window
// 3) Then we simulate a sliding window of the length of s1 and if at any point the characters in the window are the same as s1, we return true
// 4) Otherwise we return false at the end

use std::collections::HashMap;

impl Solution {
    pub fn calc_freq(s: &String) -> HashMap<char,i32> {
        let mut map = HashMap::new();
        for ch in s.chars() {
            let mut val = map.entry(ch).or_insert(0);
            *val += 1;
        }
        map
    }
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let mut freq1: HashMap<char,i32> = Solution::calc_freq(&s1);
        let mut freq2: HashMap<char,i32> = HashMap::new();
        let mut start = 0;
        for (i,ch) in s2.char_indices() {
            let mut v = freq2.entry(ch).or_insert(0);
            *v += 1;
            if(i - start > s1.len() -1){
                let mut val = freq2.entry(s2.chars().nth(start).unwrap()).or_default();
                *val -= 1;
                if *val == 0 {freq2.remove(&s2.chars().nth(start).unwrap());}
                start += 1;
            }
            if(freq1 == freq2){return true;}
        }
        false
    }
}
