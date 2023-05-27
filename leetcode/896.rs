/*
Question:
An array is monotonic if it is either monotone increasing or monotone decreasing.
An array nums is monotone increasing if for all i <= j, nums[i] <= nums[j]. An array nums is monotone decreasing if for all i <= j, nums[i] >= nums[j].
Given an integer array nums, return true if the given array is monotonic, or false otherwise.
*/

// Approach:
// 1) We find out the tone of the array first by checking if the difference between the first elements is negetive or positive
// 2) Then we loop through the array, checking the sign of the difference of adjacent elements 
//    and if at any point it is not equal to tone then we return false
// 3) Otherwise we return true at the end

impl Solution {
    pub fn is_monotonic(nums: Vec<i32>) -> bool {
        let mut tone = nums.last().unwrap() - nums[0];
        tone = if tone >= 0 {1} else {-1};
        for i in 1..nums.len() {
           let mut sign = nums[i]-nums[i-1];
           if sign == 0 {continue;}
           sign = sign/sign.abs();
           if sign != tone {return false;}
        }
        true
    }
}
