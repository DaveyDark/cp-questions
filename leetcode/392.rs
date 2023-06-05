/*
Question:
Given two strings s and t, return true if s is a subsequence of t, or false otherwise.
A subsequence of a string is a new string that is formed from the original string by deleting some (can be none) of the characters,
without disturbing the relative positions of the remaining characters. (i.e., "ace" is a subsequence of "abcde" while "aec" is not).
*/

// Approach:
// 1) We turn s and t into vectors and make ch to keep track of the last char of s
// 2) Then we loop until t is empty, emoving the last character each iteration
//    If the removed char mathes ch then we also remove a char from s and reset ch
// 3) At the end we check if s is empty, if it is then all chars were matched and found in t
//    Therefore it is a subsequence and we return true

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut s: Vec<char> = s.chars().collect();
        let mut t: Vec<char> = t.chars().collect();
        let mut ch = *s.last().unwrap_or(&'`');
        while !t.is_empty() {
            let c = t.pop().unwrap_or('~');
            if c == ch {
                s.pop();
                ch = *s.last().unwrap_or(&'`');
            }
        }
        s.is_empty()
    }
}
