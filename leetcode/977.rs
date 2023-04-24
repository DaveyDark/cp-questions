/*
Question:
Given an integer array nums sorted in non-decreasing order, return an array of the squares of each number sorted in non-decreasing order.
*/

// Approach:
// 1) We make two ints to track the left and right point of the vector and a new vector vec to store the solution vector
// 2) We compare abs value of left and right and insert the square of the bigger number into the vec vector and update the index tracker accordingly
// 3) We return the vec vector at the end

impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut left: i32 = 0;
        let mut right: i32 = nums.len() as i32 -1;
        let mut vec: Vec<i32> = Vec::new();
        while left <= right {
            if nums[left as usize].abs() > nums[right as usize].abs() {
                vec.insert(0,nums[left as usize]*nums[left as usize]);
                left += 1;
            } else {
                vec.insert(0,nums[right as usize]*nums[right as usize]);
                right -= 1;
            }
        }
        vec
    }
}
