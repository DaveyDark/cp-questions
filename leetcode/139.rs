/*
Question:
Given a string s and a dictionary of strings wordDict, return true if s can be segmented into a space-separated sequence of one or more dictionary words.

Note that the same word in the dictionary may be reused multiple times in the segmentation.
*/

// Approach
// 1) Make a vec of bools that will store weather s[..i] is breakable into dict words
// 2) Set the first value of breakable to  true
// 3) Loop for the length of s
//      Loop an index j from 0 to i
//        If a substring s[..j] is breakable and s[j..i] is in dict, s[..i] is breakable
//        So check the above condition adn if it's true, we set breakable[i] = true and break
// 4) At the end, we can check the last item of breakable to see if the string is breakable

use std::collections::HashSet;

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let mut breakable = vec![false; s.len() + 1];
        breakable[0] = true;
        for i in 1..s.len()+1 {
            for j in 0..i {
                if breakable[j] && word_dict.contains(&s[j..i].to_string()) {
                    breakable[i] = true;
                    break;
                }
            }
        }
        breakable[s.len()]
    }
}
