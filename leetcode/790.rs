/*
You have two types of tiles: a 2 x 1 domino shape and a tromino shape. You may rotate these shapes.

Given an integer n, return the number of ways to tile an 2 x n board. Since the answer may be very large, return it modulo 109 + 7.

In a tiling, every square must be covered by a tile. 
Two tilings are different if and only if there are two 4-directionally adjacent cells on the board such that 
exactly one of the tilings has both squares occupied by a tile.
*/

// Approach:
// The dp pattern observed for this is that dp[i] is the sum of dp[i-1]*2 + dp[i-3]
// 1) Make a dp array of size n or 2, whichever is larger
// 2) Init the first two indices with 1 and 2
// 3) Go thorugh the rest of the indicies and use the pattern to assign values
//    Also modulo the value with 10^9 + 7 as specified in the question
// 4) Return the nth element of dp

impl Solution {
    pub fn num_tilings(n: i32) -> i32 {
        let mut dp = vec!(0u32; 2usize.max(n as usize));
        dp[0] = 1;
        dp[1] = 2;
        for i in 2..n as usize {
            dp[i] = (dp[i-1] * 2) + *dp.get(i-3).unwrap_or(&1);
            dp[i] = dp[i] % (10u32.pow(9) + 7);
        }
        dp[n as usize - 1] as i32
    }
}
