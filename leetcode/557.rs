/*
Question:
Given a string s, reverse the order of characters in each word within a sentence while still preserving whitespace and initial word order.
*/

// Approach:
// 1) Split the input string by whitespaces
// 2) Map the split array to reverse each word
// 3) Collect the reveresed words into a vector and join them by a whitespace

impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split_whitespace()
            .map( |word| 
                word.chars().rev().collect()
            )
            .collect::<Vec<String>>()
            .join(" ")
    }
}
