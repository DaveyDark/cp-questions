/*
Question:
You are given a string num, representing a large integer. 
Return the largest-valued odd integer (as a string) that is a non-empty substring of num, 
or an empty string "" if no odd integer exists.

A substring is a contiguous sequence of characters within a string.
*/

// Approach:
// Remove all even numbers from the end of the string by trimming it and return the resulting
// string

impl Solution {
    pub fn largest_odd_number(num: String) -> String {
        num.trim_end_matches(&['0','2','4','6','8']).to_string()
    }
}
