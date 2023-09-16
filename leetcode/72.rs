/*
Question:
Given two strings word1 and word2, return the minimum number of operations required to convert word1 to word2.

You have the following three operations permitted on a word:
    Insert a character
    Delete a character
    Replace a character
*/

// Approach:
// 1) Convert the strings to ascii bytes
// 2) Make a dp vector containing word1.len()+1 rows and word2.len()+1 columns
// 3) Fill the first row and column with the row/column numbers
// 4) Go through the remaining cells in the dp matrix
//      If the current chars are equal we set it to the top left diagonal element
//      Otherwise we set it to the min of the top left, top and left cells + 1
// 5) Return the bottom right element of the dp matrix

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let word1 = word1.into_bytes();
        let word2 = word2.into_bytes();
        let mut dp = vec!(vec!(0; word2.len() + 1); word1.len() + 1);
        for i in 0..=word1.len() {
            dp[i][0] = i as i32;
        }
        for j in 0..=word2.len() {
            dp[0][j] = j as i32;
        }
        for i in 1..=word1.len() {
            for j in 1..=word2.len() {
                if word1[i-1] == word2[j-1] {
                    dp[i][j] = dp[i-1][j-1];
                } else {
                    dp[i][j] = dp[i-1][j-1].min(dp[i-1][j]).min(dp[i][j-1]) + 1;
                }
            }
        }
        dp[word1.len()][word2.len()]
    }
}
