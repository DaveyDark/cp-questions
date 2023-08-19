/*
Question:
You are given an integer array cost where cost[i] is the cost of ith step on a staircase. 
Once you pay the cost, you can either climb one or two steps.

You can either start from the step with index 0, or the step with index 1.

Return the minimum cost to reach the top of the floor.
*/

// Approach:
// 1) Make a dp vector with the same length as cost
// 2) Init the first two elements of dp with the first two elements of cost
// 3) Go over the remaining elements and set each index to the cost of current step plus the minimum of the last or the second last step
// 4) Return the minimum of the last and second last element of the dp array

impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let mut dp = vec!(0; cost.len());
        dp[0] = cost[0];
        dp[1] = cost[1];
        for i in 2..cost.len() {
            dp[i] = cost[i] + dp[i-1].min(dp[i-2]);
        }
        dp[dp.len()-1].min(dp[dp.len()-2])
    }
}
