/*
Question:
Given an integer array nums, return true if any value appears at least twice in the array, and return false if every element is distinct.
*/

// Approach:
// 1) Sort the vector
// 2) Set lastNum = first index
// 3) Loop through indicies 1 to len-1
// 4) if current index == lastNum then return true
// 5) otherwise set lastNum to current index at the end of the iteration
// 6) return false if we reach the end of function

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut nums = nums.clone();
        nums.sort();
        let mut lastNum = nums[0];
        for i in 1..nums.len() {
            if nums[i] == lastNum {
                return true
            }
            lastNum = nums[i];
        }
        false
    }
}
