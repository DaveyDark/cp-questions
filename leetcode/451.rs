/*
Question:
Given a string s, sort it in decreasing order based on the frequency of the characters.
The frequency of a character is the number of times it appears in the string.

Return the sorted string. If there are multiple answers, return any of them.
*/

// Approach:
// 1) Create a hashmap to store the frequency of each character.
// 2) Populate the hashmap with the frequency of each character.
// 3) Convert the hashmap into a vector of tuples.
// 4) Sort the vector in descending order of frequency.
// 5) Create a string to store the answer.
// 6) Iterate through the vector and append the character to the answer string the number of times equal to its frequency.
// 7) Return the answer string.

use std::collections::HashMap;

impl Solution {
    pub fn frequency_sort(s: String) -> String {
        let mut freq = HashMap::new();
        s.chars().for_each(|c| *freq.entry(c).or_insert(0) += 1);
        let mut vec = freq.into_iter().collect::<Vec<(char, i32)>>();
        vec.sort_unstable_by_key(|e| -e.1);
        let mut ans = String::new();
        for (c, i) in vec {
            ans += &c.to_string().repeat(i as usize);
        }
        ans
    }
}
