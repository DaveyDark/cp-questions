/*
Question:
Given an array of integers nums which is sorted in ascending order, and an integer target,
write a function to search target in nums. If target exists, then return its index. Otherwise, return -1.

You must write an algorithm with O(log n) runtime complexity.
*/

// Approach:
// 1) Set left to first index and right to last index
// 2) loop while left > right
// 3) Calculate mid point = left+right / 2
// 3) Compare nums[mid] to target and update left or right accordingly, or return the index of nums[mid] == target

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left: i32 = 0;
        let mut right: i32 = nums.len() as i32 - 1;
        let mut mid: i32 = 0;


        loop {
            mid = (left+right)/2;
            println!("{} + {} / 2 = {}",left,right,mid);
            if nums[mid as usize] == target {
                return mid
            } else if nums[mid as usize] < target {
                left = mid+1;
            } else {
                right = mid-1;
            }
            if left > right {
                break;
            }
        }
        -1
    }
}
