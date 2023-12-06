/*
Question:
Hercy wants to save money for his first car. He puts money in the Leetcode bank every day.

He starts by putting in $1 on Monday, the first day. 
Every day from Tuesday to Sunday, he will put in $1 more than the day before. 
On every subsequent Monday, he will put in $1 more than the previous Monday.

Given n, return the total amount of money he will have in the Leetcode bank at the end of the nth day.
*/

// Approach:
// 1) Init a sum counter
// 2) Loop from 1 to n and each iteration add i/7 to sum and i%7 if it is non zero or 6 if i%7 is 0
// 3) Return sum

impl Solution {
    pub fn total_money(n: i32) -> i32 {
        let mut sum = 0;
        for i in 1..=n {
            sum += (i/7);
            sum += if i%7==0 {6} else {i%7};
        }
        sum
    }
}
