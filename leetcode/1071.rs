/*
Question:
For two strings s and t, we say "t divides s" if and only if s = t + ... + t (i.e., t is concatenated with itself one or more times).
Given two strings str1 and str2, return the largest string x such that x divides both str1 and str2.
*/

// Approach:
// 1) We get the shorter of the two string and then iterate through it's substrings in reverse order
// 2) Then each iteration we check if the substring can divide both the strings, and return the substring as a string if it can
// 3) Otherwise we return an empty string at the end

impl Solution {
    pub fn can_divide(str1: &str, str2: &str) -> bool { str2.repeat(str1.len()/str2.len()) == str1 }
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        let shorter = if str1.len() < str2.len() {&str1} else {&str2};
        for i in (1..=shorter.len()).rev() {
            if Self::can_divide(&str1,&str1[0..i]) && 
            Self::can_divide(&str2,&str1[0..i]) {
                 return str1[0..i].to_string() 
            }
        }
        String::new()
    }
}
