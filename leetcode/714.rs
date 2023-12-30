/*
Question:
You are given an array prices where prices[i] is the price of a given stock on the ith day,
and an integer fee representing a transaction fee.

Find the maximum profit you can achieve.
You may complete as many transactions as you like,
but you need to pay the transaction fee for each transaction.

Note:
    You may not engage in multiple transactions simultaneously (i.e., you must sell the stock before you buy again).
    The transaction fee is only charged once for each stock purchase and sale.
*/

// Approach:
// 1) Maintain two states: 0 - not holding, 1 - holding
// 2) Make two arrays to store the states for current and previous day
// 3) Loop through the prices and update the states
//    For state 0, we can either hold the state or sell the stock
//    For state 1, we can either hold the state or buy the stock
//    Then make the current states the last and reset the curr states
// 4) Return the max of the two states in last

impl Solution {
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        let mut curr = vec![0, 0];
        let mut last = vec![0, 0];
        last[1] = -prices[0] - fee;

        for i in 1..prices.len() {
            curr[0] = last[0].max(last[1] + prices[i]);
            curr[1] = last[1].max(last[0] - prices[i] - fee);
            last = curr;
            curr = vec![0, 0];
        }

        last[0].max(last[1])
    }
}
