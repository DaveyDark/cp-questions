/*
Question:
A peak element is an element that is strictly greater than its neighbors.

Given a 0-indexed integer array nums, find a peak element, and return its index. If the array contains multiple peaks, return the index to any of the peaks.

You may imagine that nums[-1] = nums[n] = -∞. In other words, an element is always considered to be strictly greater than a neighbor that is outside the array.

You must write an algorithm that runs in O(log n) time.
*/

// Approach:
// We do a simple binary search and compare the mid element to the next one

impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let mut left: i32 = 0;
        let mut right: i32 = nums.len() as i32 -1;
        let mut mid: i32 = 0;
        while left < right {
            mid = left + (right-left)/2;
            if nums[mid as usize] > nums[mid as usize + 1] {
                right = mid;
            } else {
                left = mid+1;
            }
        }
        right
    }
}
