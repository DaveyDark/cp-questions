/*
Question:
Given a string s, partition the string into one or more substrings such that the characters in each substring are unique. That is, no letter appears in a single substring more than once.

Return the minimum number of substrings in such a partition.

Note that each character should belong to exactly one substring in a partition.
*/


// Approach:
// 1) Keep another string tos tore each substring
// 2) Keep a counter and start it at 1(Because there is always at least 1 substring)
// 3) While there are letters in the string, we will remove them one by one and add to the substring
// 4) If the substring already contains the letter, we will increase the counter and clear the substring

impl Solution {
    pub fn partition_string(s: String) -> i32 {
        let mut substring: String = String::new();
        let mut s: String = s.clone(); // make s mutable
        let mut cnt = 1;
        while s.len() > 0 {
            let ch: char = s.pop().unwrap(); // get last letter
            if substring.contains(ch) {
                //start new slice
                substring.clear();
                cnt += 1;
            }
            //add to old slice
            substring.push(ch);
        }
        cnt 
    }
}
