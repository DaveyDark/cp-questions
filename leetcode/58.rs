/*
Question:
Given a string s consisting of words and spaces, return the length of the last word in the string.
A word is a maximal substring consisting of non-space characters only.
*/

// Approach:
// We remove whitespace from the edges, split the string by spaces and then return the length of the last element of the obtained vector

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        s.trim().split(' ').last().unwrap_or("").len() as i32
    }
}
