/*
Question:
Given a string s, return the string after replacing every uppercase letter with the same lowercase letter.709
*/

// Approach:
// We can just return the result of to_lowercase() built-in function of the String struct

impl Solution {
    pub fn to_lower_case(s: String) -> String {
        s.to_lowercase()
    }
}
