/*
Question:
Given a string s of zeros and ones,
return the maximum score after splitting the string into two non-empty substrings
(i.e. left substring and right substring).

The score after splitting a string is the number of zeros in the left substring
plus the number of ones in the right substring.
*/

// Approach:
// 1) Transform the string into a vector of chars.
// 2) Maintain a prefix sum of 0s and suffix sum of 1s.
// 3) Iterate over the string and update the prefix and suffix sum.
// 4) Return the maximum sum.

impl Solution {
    pub fn max_score(s: String) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let mut prefix = 0;
        let mut suffix = s.iter().filter(|&&c| c == '1').count() as i32;
        let mut sum = 0;
        for c in 0..s.len() - 1 {
            match s[c] {
                '0' => prefix += 1,
                '1' => suffix -= 1,
                _ => (),
            }
            sum = sum.max(prefix + suffix);
        }
        sum
    }
}
