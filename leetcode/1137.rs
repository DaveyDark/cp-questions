/*
Question:
The Tribonacci sequence Tn is defined as follows: 

T0 = 0, T1 = 1, T2 = 1, and Tn+3 = Tn + Tn+1 + Tn+2 for n >= 0.

Given n, return the value of Tn.
*/

// Approach:
// 1) Make a dp vector the size of n+1
// 2) Return n if it's <= 1
// 3) Init dp[1] with 1
// 4) Loop from 2 to n(inclusive) and calculate dp[i] by summing dp[i-1], dp[i-2] and dp[i-3]
// 5) Return dp[n]

impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        let mut dp = vec!(0; n as usize + 1);
        if n < 1 {return 0;}
        dp[1] = 1;
        for i in 2..=n as usize {
            dp[i] = dp[i-1] + dp[i-2] + *dp.get(i-3).unwrap_or(&0);
        }
        dp[n as usize]
    }
}
