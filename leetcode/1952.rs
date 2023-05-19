/*
Question:
Given an integer n, return true if n has exactly three positive divisors. Otherwise, return false.
An integer m is a divisor of n if there exists an integer k such that n = k * m.
*/

// Approach:
// We just loop from 1 to n and add 1 to div for every number that can divide n perfectly and then return true of the number is exactly 3

impl Solution {
    pub fn is_three(n: i32) -> bool {
        let mut div: i32 = 0;
        for i in 1..=n {
            if n%i == 0 {div+=1;}
        }
        div==3
    }
}
