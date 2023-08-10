/*
Question:
Given an array nums of size n, return the majority element.

The majority element is the element that appears more than ⌊n / 2⌋ times. You may assume that the majority element always exists in the array.
*/

// Approach:
// 1) Clone and sort the given array
// 2) Return the element at len/2 index

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut nums = nums.clone();
        nums.sort_unstable();
        nums[nums.len()/2]
    }
}
