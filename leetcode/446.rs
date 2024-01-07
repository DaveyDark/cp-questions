/*
Question:
Given an integer array nums, return the number of all the arithmetic subsequences of nums.

A sequence of numbers is called arithmetic if it consists of at least three elements
and if the difference between any two consecutive elements is the same.
    For example, [1, 3, 5, 7, 9], [7, 7, 7, 7], and [3, -1, -5, -9] are arithmetic sequences.
    For example, [1, 1, 2, 5, 7] is not an arithmetic sequence.

A subsequence of an array is a sequence that can be formed by removing some elements (possibly none) of the array.
    For example, [2,5,10] is a subsequence of [1,2,1,2,4,1,5,10].

The test cases are generated so that the answer fits in 32-bit integer.
*/

// Approach:
// 1) Create a HashMap to store the indices of all numbers
// 2) Create a DP matrix of size n x n
// 3) Populate the map with the indices of all numbers
// 4) Iterate over the matrix and for each pair of indices (i, j) where i < j
//      For each pair, we find the number which comes before nums[i] and nums[j] in the AP.
//      a = 2b - c
//      where a is the required number, b is nums[i] and c is nums[j]
//      If this number is in the map, we update dp[i][j] with all the pairs that can be formed for
//      this AP. We do this by getting all indices in the map x < i and adding 1 to their value in dp
//      with the pair of dp[x][i]
//      Finally, we add dp[i][j] to the sum
// 5) Return the sum

use std::collections::HashMap;

impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        let mut dp = vec![vec![0; nums.len()]; nums.len()];
        nums.iter()
            .enumerate()
            .for_each(|(i, n)| map.entry(*n as i64).or_insert(Vec::new()).push(i));

        let mut sum = 0;
        for i in 1..nums.len() {
            for j in i + 1..nums.len() {
                match map.get(&(2 * nums[i] as i64 - nums[j] as i64)) {
                    Some(l) => l
                        .iter()
                        .take_while(|&&x| x < i)
                        .for_each(|&x| dp[i][j] += dp[x as usize][i] + 1),
                    _ => (),
                }
                sum += dp[i][j];
            }
        }

        sum
    }
}
