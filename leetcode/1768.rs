/*
Question:
You are given two strings word1 and word2. Merge the strings by adding letters in alternating order, starting with word1. 
If a string is longer than the other, append the additional letters onto the end of the merged string.
Return the merged string.
*/

// Approach:
// 1) We loop from 0 to the bigger of the two word lengths
// 2) Each iteration we first check if word1 has a char at i and add it to the solution string if it does
//    Then we do the same for word2
// 3) We return the Solution string at the end

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut sol = String::new();
        for i in 0..(if word1.len() > word2.len() {word1.len()} else {word2.len()}) {
            match word1.chars().nth(i) {
                Some(ch) => sol.push(ch),
                None => {}
            }
            match word2.chars().nth(i) {
                Some(ch) => sol.push(ch),
                None => {}
            }
        }
        sol
    }
}
