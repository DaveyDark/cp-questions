/*
Given a string s, return the length of the longest substring between two equal characters,
excluding the two characters. If there is no such substring return -1.

A substring is a contiguous sequence of characters within a string.
*/

// Approach:
// 1) Make an array of size 26 to store the first index of each character and initialize it with None.
//    Also, make a variable len to store the maximum length and initialize it with -1.
// 2) Iterate over the characters in the string
//    If the current character is already present in the array, update len with the max of the
//    difference of i and
//    Otherwise, update the array with the index
// 3) Return the len

impl Solution {
    pub fn max_length_between_equal_characters(s: String) -> i32 {
        let mut chars = vec![None; 26];
        let mut len = -1;
        for (i, ch) in s.bytes().enumerate() {
            if let Some(j) = chars[(ch - b'a') as usize] {
                len = len.max(i as i32 - j - 1);
            } else {
                chars[(ch - b'a') as usize] = Some(i as i32);
            }
        }
        len
    }
}
