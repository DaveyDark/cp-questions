/*
Question:
Given a string s, return the number of unique palindromes of length three that are a subsequence of s.
Note that even if there are multiple ways to obtain the same subsequence, it is still only counted once.
A palindrome is a string that reads the same forwards and backwards.

A subsequence of a string is a new string generated from the original string with some characters (can be none) 
deleted without changing the relative order of the remaining characters.
    For example, "ace" is a subsequence of "abcde".
*/

// Approach:
// 1) Covert the string s into a byte array for easier retrieval
// 2) Loop over all the bytes from a to z
//      For each letter we use two pointers to look for the matching letter in the string from both
//      ends. Once we find it, we calculate the number of unique characters between the two points
//      and add those to the sum
// 3) Return the calculated sum

use std::collections::HashSet;

impl Solution {
    pub fn count_palindromic_subsequence(s: String) -> i32 {
        let mut sum = 0;
        let mut chars = s.into_bytes();

        for c in b'a'..=b'z' {
            let mut start = 0;
            let mut end = chars.len() -1;
            while start < end {
                if chars[start] == c && chars[end] == c {
                    let mut set = HashSet::new();
                    for i in start+1..end {
                        set.insert(chars[i]);
                    }
                    sum += set.len() as i32;
                    break;
                } else if chars[start] == c {
                    end -= 1;
                } else {
                    start += 1;
                }
            }
        }
        
        sum
    }
}
