/*
Question:
You are given an array prices where prices[i] is the price of a given stock on the ith day.
You want to maximize your profit by choosing a single day to buy one stock and choosing a different day in the future to sell that stock.
Return the maximum profit you can achieve from this transaction. If you cannot achieve any profit, return 0.
*/

// Approach
// 1) We set buying and  selling price to the first index and max_profit to 0
// 2) We loop through the prices vector
// 3) If current price is less than our buying price then we set buying price to current price
// 4) If it is more than the buying price, we check to see if we can earn a greater profit than max_profit by selling at this price
//    If we can then we set max_profit to the profit gained by selling at this day
// 5) We return the max_profit

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut buy = prices[0];
        let mut sell = prices[0];
        let mut max_profit = 0;
        for price in prices {
            if price < buy {
                buy = price;
            }
            if price > buy && price - buy > max_profit {
                max_profit = price - buy;
            }
        }
        max_profit
    }
}
