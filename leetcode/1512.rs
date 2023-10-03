/*
Question:
Given an array of integers nums, return the number of good pairs.
A pair (i, j) is called good if nums[i] == nums[j] and i < j.
*/

// Approach:
// 1) Make counter to track good pairs
// 2) Loop over all indices in the array
//      Loop over all indices grater than the current index
//          if nums[i] == nums[j] increment the counter
// 3) Return the counter

impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut cnt = 0;
        for i in 0..nums.len() {
            for j in i+1..nums.len() {
                if nums[i] == nums[j] {cnt += 1}
            }
        }
        cnt
    }
}
