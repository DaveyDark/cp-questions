/*
Question:
Given an integer n, return an array ans of length n + 1 such that for each i (0 <= i <= n), ans[i] is the number of 1's in the binary representation of i.
*/

// Approach:
// 1) Make a vector to store the solution
// 2) Loop through numbers from 0 to n
//      If a number is odd the number of bits will increase
//      Otherwise the number of bits will decrease by the number of trailing zeroes - 1
//      Then we push the current 1 bit count to the vector
// 3) Return the vector at the end

impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut bits = Vec::new();
        let mut cnt = 0;
        for i in 0..=n {
            if i%2 == 1 {cnt += 1}
            else if i>0 {
                cnt -= i.trailing_zeros() - 1;
            }
            bits.push(cnt as i32);
        }
        bits
    }
}
