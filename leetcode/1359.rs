/*
Question:
Given n orders, each order consist in pickup and delivery services. 
Count all valid pickup/delivery possible sequences such that delivery(i) is always after of pickup(i). 
Since the answer may be too large, return it modulo 10^9 + 7.
*/

// Approach:
// 1) Make a dp vector of i64s containing n elements init to 0
// 2) Init the first element to 1
// 3) Loop through the remaining elements in the array
//     Calculate dp[i] by using the formula dp[i-1] * (i+1) * ((i*2)+1)
//     Modulo the calculated value with 10^9 + 7
// 4) Return the last element of the dp vector

impl Solution {
    pub fn count_orders(n: i32) -> i32 {
        let mut dp = vec!(0i64; n as usize);
        dp[0] = 1;
        for i in 1..dp.len() {
            dp[i] = dp[i-1] as i64 * (i as i64 + 1) * ((i as i64 * 2) + 1);
            dp[i] = dp[i] % (10i64.pow(9) + 7);
        }
        dp[dp.len()-1] as i32
    }
}
