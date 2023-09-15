/*
Question:
Given two strings text1 and text2, return the length of their longest common subsequence. If there is no common subsequence, return 0.

A subsequence of a string is a new string generated from the original string with some characters (can be none) deleted without changing the relative order of the remaining characters.
    For example, "ace" is a subsequence of "abcde".

A common subsequence of two strings is a subsequence that is common to both strings.
*/

// Approach:
// 1) Convert text1 and text2 to ascii byte vectors
// 2) Create a dp vector with text2.len() columns and text1.len() rows
// 3) Loop through every cell in the dp vector
//      If the i-th char of text1 and j-th char of text2 are equal, set dp[i][j] to dp[i-1][j-1] +1
//      Otherwise set to the greater of the top of left adjacent element
// 4) Return the last(bottom right) element of the dp vector

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let text1 = text1.into_bytes();
        let text2 = text2.into_bytes();
        let mut dp = vec!(vec!(0; text2.len()); text1.len());
        for i in 0..text1.len() {
            for j in 0..text2.len() {
                if text1[i] == text2[j] {
                    dp[i][j] = *dp.get(i-1).unwrap_or(&vec!()).get(j-1).unwrap_or(&0) + 1;
                } else {
                    dp[i][j] = *dp[i].get(j-1).unwrap_or(&0)
                    .max(dp.get(i-1).unwrap_or(&vec!()).get(j).unwrap_or(&0));
                }
            }
        }
        dp[text1.len()-1][text2.len()-1]
    }
}
