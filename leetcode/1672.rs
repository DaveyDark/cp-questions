/*
Question:
You are given an m x n integer grid accounts where accounts[i][j] is the amount of money the ith customer has in the jth bank. Return the wealth that the richest customer has.
A customer's wealth is the amount of money they have in all their bank accounts. The richest customer is the customer that has the maximum wealth.
*/

// Appraoch:
// I used a function chain to solve it with a one liner.
// We first map through the rows of accounts and return the sum of the elements of the row, which gives us an iterator containing the sums of all rows
// Then we calculate the max element of the iterator returned, which gives up the maximum wealth, and unwrap and return it

impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        accounts.iter().map(|row| row.iter().sum() ).max().unwrap()
    }
}
