/*
Question:
You are given an array of integers nums, there is a sliding window of size k which is moving from the very left of the array to the very right. 
You can only see the k numbers in the window. Each time the sliding window moves right by one position.

Return the max sliding window.
*/

// Approach 1(Deque)(Editorial)(Better runtime):
// 1) Create a deque dq<usize> and a Vec<i32> to store the answer.
// 2) Iterate over the first k elements from i = 0 to k - 1
//      While dq is not empty and the current element nums[i] is greater or equal to last element, continue to pop the last element.
//      Push i at the end of dq.
// 3) Push the largest element of the first window nums[dq.last()] to the answer.
// 4) Iterate over all the remaining elements from k to the end of the vector to move to the next windows
//    Check if the element at the front of dq is equal to i - k. If it is equal to i - k, it cannot be included in the current window, pop this element.
//    While dq is not empty and the current element nums[i] is greater or equal to nums[dq.back()], continue to pop the last element.
//    Push i at the end of dq.
//    Push the largest element of the current window nums[dq.peekFirst()] to the answer.
// 5) Return the ans vector

use std::collections::VecDeque;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut dq: VecDeque<usize> = VecDeque::new();
        let mut max_win = vec!();
        for i in 0..k as usize {
            while !dq.is_empty() && nums[i] >= nums[*dq.back().unwrap()] {
                dq.pop_back();
            }
            dq.push_back(i);
        }
        max_win.push(nums[*dq.front().unwrap()]);
        for i in k as usize..nums.len() {
            if *dq.front().unwrap() == i-k as usize {
                dq.pop_front();
            }
            while !dq.is_empty() && nums[i] >= nums[*dq.back().unwrap()] {
                dq.pop_back();
            }
            dq.push_back(i);
            max_win.push(nums[*dq.front().unwrap()]);
        }
        max_win
    }
}

// Approach 2(Manually calculating max when it changes)(Better memory):
// 1) Define a function calc_max that takes a slice of an array and return the index of the max element
// 2) Make a vector to store the answer and a variable to store the index of current max element
// 3) Loop from the start of the given vector to len-k index
//    If the max_index < i, it has gone out of scope so we need to recalculate max for the slice
//    Else if the new element in this window i.e. the last one is more than the current max element, we cange max_index to i
//    Then just push the max_element for the current window to the max_window vector
// 4) Return the max_window vector

impl Solution {
    fn calc_max(nums: &[i32]) -> usize {
        let mut max = nums[0];
        let mut max_index = 0;
        for i in 0..nums.len() {
            if nums[i] >= max {
                max = nums[i];
                max_index = i;
            }
        }
        max_index
    }
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut max_window = vec!();
        let mut max_index = Self::calc_max(&nums[..k as usize]);
        for i in 0..=nums.len()-k as usize {
            if max_index < i {
                max_index = i + Self::calc_max(&nums[i..i+k as usize]);
            } else if nums[i+k as usize-1] > nums[max_index] {
                max_index = i+k as usize-1;
            }
            max_window.push(nums[max_index]);
        }
        max_window
    }
}
