/*
Question:
You are given an integer array nums consisting of n elements, and an integer k.
Find a contiguous subarray whose length is equal to k that has the maximum average value and return this value. 
Any answer with a calculation error less than 10-5 will be accepted.
*/

// Approach:
// 1) We make variables to keep stack of start and end of sliding window, the current sum and the max sum
// 2) We go over the vector and calculate the sum of the elements in the range.
//    Every iteration, the end index is added to sum and it is moved 1 step forward
//    And if the size of current window is more than the needed size, we subtract the element at start index and increment start too
//    Then we check if the current sum is more than max and is a full range sum, and set max to sum if it is
// 3) At the end we return the max sum divided by k to calculate the max average

impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let mut start: usize = 0;
        let mut end: usize = 0;
        let mut sum = 0;
        let mut max = i32::MIN;
        for i in 0..nums.len() {
            sum += nums[end];
            end += 1;
            if end - start > k as usize {
                sum -= nums[start];
                start += 1;
            }
            if sum > max && (end - start == k as usize) { max = sum; }
        }
        max as f64 / k as f64
    }
}
