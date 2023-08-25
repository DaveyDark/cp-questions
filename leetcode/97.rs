/*
Question:
Given strings s1, s2, and s3, find whether s3 is formed by an interleaving of s1 and s2.

An interleaving of two strings s and t is a configuration where s and t are divided into n and m
substrings
respectively, such that:

    s = s1 + s2 + ... + sn
    t = t1 + t2 + ... + tm
    |n - m| <= 1
    The interleaving is s1 + t1 + s2 + t2 + s3 + t3 + ... or t1 + s1 + t2 + s2 + t3 + s3 + ...

Note: a + b is the concatenation of strings a and b.
*/

// Appraoch:
// Use memoization with recursion
// 1) Make a 2d vector of (s1.len() + 1) x (s2.len() + 1) for memoization where memo[i][j] will
//    represent weather s1[i] and s2[j] can be interleaved into s3[i+j]
// 2) Call inerleave with the memoization vector and given parameters and return the result
// 3) In interleave, for the base case we check if s1 and s3 are exhaused and if they are then we 
//    check if the length of used characters is equal to length of s3 and return the result
// 4) Then we check if the result for the current calculation is already in the memo vector and
//    return it if it is present
// 5) Otherwise we check if the next character in s3 is in s1 or s2
//    We do this by recursively calling interleave if the cuurent char in s1 or s2 matches the
//    current char in s3
//    Whatever the result, we memoize it and then return it

impl Solution {
    fn interleave(
        s1: &str,
        s2: &str,
        s3: &str,
        i: usize,
        j: usize,
        memo: &mut Vec<Vec<Option<bool>>>,
    ) -> bool {
        if i >= s1.len() && j >= s2.len() {
            return i + j == s3.len();
        }
        if let Some(&res) = memo.get(i).and_then(|row| row.get(j)) {
            if let Some(r) = res {
                return r;
            }
        }
        let mut can_interleave = false;
        if s1.chars().nth(i).unwrap_or('-') == s3.chars().nth(i + j).unwrap_or(' ') {
            can_interleave = can_interleave || Self::interleave(s1, s2, s3, i + 1, j, memo);
        }
        if s2.chars().nth(j).unwrap_or('-') == s3.chars().nth(i + j).unwrap_or(' ') {
            can_interleave = can_interleave || Self::interleave(s1, s2, s3, i, j + 1, memo);
        }
        memo.get_mut(i).map(|row| row[j] = Some(can_interleave));
        can_interleave
    }
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        let mut memo = vec![vec![None; s2.len() + 1]; s1.len() + 1];
        Self::interleave(&s1, &s2, &s3, 0, 0, &mut memo)
    }
}
