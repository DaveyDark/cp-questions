/*
Question:
Given a string s, return the number of homogenous substrings of s. Since the answer may be too large, return it modulo 109 + 7.

A string is homogenous if all the characters of the string are the same.

A substring is a contiguous sequence of characters within a string.
*/

// Approach:
// 1) Init variables to store the current streak, the last char(init to the first char) and the count so far
// 2) Loop through the remaining characters in the string
//      For each char, if it is the same as last increment streak
//      Otherwise reset streak
//      Add streak to the count in either case
// 3) Return the count

impl Solution {
    pub fn count_homogenous(s: String) -> i32 {
        let mut streak = 1;
        let mut last = s.chars().next().unwrap();
        let mut count = 1;
        for ch in s.chars().skip(1) {
            if ch == last {streak += 1}
            else {
                streak = 1;
                last = ch;
            }
            count = (count + streak as u32) % (1e9 as u32 + 7);
        }
        count as i32
    }
}
