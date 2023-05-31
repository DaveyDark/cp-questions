/*
Question:
Given a string s, check if it can be constructed by taking a substring of it and appending multiple copies of the substring together.
*/

// Appraoch:
// 1) We loop through all possible substring lengths from 1 to len/2
// 2) For each length we check if it can fully divide the length
//    If it can then we check we repeating it len/i times gives us the original string
//    And return true if it does
// 3) Otherwise we return false at the end of the function

impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        for i in 1..=s.len()/2 {
            if s.len()%i==0 {
                if s[0..i].repeat(s.len()/i) == s {return true;}
            }
        }
        false
    }
}
