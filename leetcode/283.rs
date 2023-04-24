/*
Given an integer array nums, move all 0's to the end of it while maintaining the relative order of the non-zero elements.
Note that you must do this in-place without making a copy of the array.
*/

// Approach:
// 1) We go through the array in reverse order
// 2) If the current element is 0 we push it to the end of the array

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut x;
        for i in (0..nums.len()).rev() {
            if nums[i] == 0{
                x = nums.remove(i);
                nums.push(x);
            } 
        }
    }
}
