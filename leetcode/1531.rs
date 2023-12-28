/*
Question:
Run-length encoding is a string compression method that works by replacing consecutive identical characters
(repeated 2 or more times) with the concatenation of the character and
the number marking the count of the characters (length of the run).
For example, to compress the string "aabccc" we replace "aa" by "a2" and replace "ccc" by "c3".
Thus the compressed string becomes "a2bc3".

Notice that in this problem, we are not adding '1' after single characters.

Given a string s and an integer k.
You need to delete at most k characters from s such that the run-length encoded version of s has minimum length.

Find the minimum length of the run-length encoded version of s after deleting at most k characters.
*/

// Approach:
// 1) Make a DP function that takes the string, k, i, last, cnt, j and memo
// 2) If i == s.len() return 0
// 3) If memo contains the value, return it
// 4) Otherwise, calculate the value by taking the current character and recursing on the remaining string
// 5) If the last character is same as current character, then we can either remove the character
//    or keep it, making recursive calls for both the cases
// 6) if j < k, then we can remove the character and recurse on the remaining string
// 7) Insert the value in memo and return it
// 8) In main function, call the dp function with initial values

use std::collections::HashMap;

impl Solution {
    fn dp(
        s: &[u8],
        k: usize,
        i: usize,
        last: u8,
        cnt: usize,
        j: usize,
        memo: &mut HashMap<(usize, u8, usize, usize), i32>,
    ) -> i32 {
        if i == s.len() {
            0
        } else if let Some(additional_len) = memo.get(&(i, last, cnt, j)) {
            *additional_len
        } else {
            let b = s[i];
            let mut rez = 1 + Self::dp(s, k, i + 1, b, 1, j, memo);
            if last == b {
                let comp = if cnt != 1 && cnt != 9 && cnt != 99 {
                    Self::dp(s, k, i + 1, b, cnt + 1, j, memo)
                } else {
                    1 + Self::dp(s, k, i + 1, b, cnt + 1, j, memo)
                };
                rez = rez.min(comp);
            }
            if j < k {
                rez = rez.min(Self::dp(s, k, i + 1, last, cnt, j + 1, memo));
            }
            memo.insert((i, last, cnt, j), rez);
            rez
        }
    }

    pub fn get_length_of_optimal_compression(s: String, k: i32) -> i32 {
        Self::dp(s.as_bytes(), k as usize, 0, b'A', 0, 0, &mut HashMap::new())
    }
}
