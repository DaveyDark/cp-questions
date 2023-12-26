/*
Question:
You have n dice, and each die has k faces numbered from 1 to k.

Given three integers n, k, and target,
return the number of possible ways (out of the kn total ways) to roll the dice,
so the sum of the face-up numbers equals target.
Since the answer may be too large, return it modulo 109 + 7.
*/

// Approach:
// 1) Handle edge cases(when there is 1 or 0 possible combinations)
// 2) Create a dp table of size n * target.
// 3) Initialize the first row with k 1s.
// 4) Iterate over each cell and calculate the value
//    To calculate the value, sum the last k values in the previous row.
//    Also MOD the value with 1_000_000_007.
// 5) Return the last value in the last row.

const MOD: i64 = 1_000_000_007;

impl Solution {
    pub fn num_rolls_to_target(n: i32, k: i32, target: i32) -> i32 {
        if n * k < target || target < n {
            return 0;
        }
        if n * k == target || n == target {
            return 1;
        }
        let (n, k, target) = (n as usize, k as usize, target as usize);
        let mut dp = vec![vec![0; target]; n];
        for i in 0..target.min(k) {
            dp[0][i] = 1;
        }
        for i in 1..dp.len() {
            for j in i..dp[0].len() {
                for k in 0..k {
                    dp[i][j] += *dp[i - 1].get(j - k - 1).unwrap_or(&0);
                    dp[i][j] %= MOD;
                }
            }
        }
        dp[n - 1][target - 1] as i32
    }
}
