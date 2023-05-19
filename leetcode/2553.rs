/*
Question:
Given an array of positive integers nums, return an array answer that consists of the digits of each integer in nums after separating them in the same order they appear in nums.
To separate the digits of an integer is to get all the digits it has in the same order.
    For example, for the integer 10921, the separation of its digits is [1,0,9,2,1].
*/

// Approach:
// 1) We use a method called push_digits that returns a vector of the digits of the input number
// 2) Then we call this method on all the numbers in nums and append the returned vector to the solution vector and return it at the end

impl Solution {
    pub fn push_digits(n: i32) -> Vec<i32> {
        let mut n = n;
        let mut digits = Vec::new();
        while n > 0 {
            digits.insert(0, n%10);
            n/=10;
        }
        digits
    }
    pub fn separate_digits(nums: Vec<i32>) -> Vec<i32> {
        let mut digits = Vec::new();
        for n in nums {
            digits.append(&mut Self::push_digits(n));
        }
        digits
    }
}
