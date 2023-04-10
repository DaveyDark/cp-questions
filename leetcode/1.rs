/*
Question:
Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
You may assume that each input would have exactly one solution, and you may not use the same element twice.
You can return the answer in any order.
*/

// Approach:
// 1) Loop through all indicies of nums
// 2) A second nested loop from n+1 to last index
// 3) Return n and m if the sum of their values is target

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for n in 0..nums.len() {
            for m in n+1..nums.len() {
                if nums[n] + nums[m] == target {
                    return vec!(n as i32,m as i32);
                }
            }
        }
        vec!(0,0)
    }
}
