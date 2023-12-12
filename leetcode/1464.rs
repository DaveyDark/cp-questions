/*
Question:
Given the array of integers nums, you will choose two different indices i and j of that array. Return the maximum value of (nums[i]-1)*(nums[j]-1).
*/

// Approach:
// 1) Sort the array
// 2) Return the product of the last two elements

impl Solution {
    pub fn max_product(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        (nums[nums.len() - 1] - 1) * (nums[nums.len() - 2] - 1)
    }
}
