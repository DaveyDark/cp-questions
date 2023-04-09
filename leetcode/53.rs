/*
Question:
Given an integer array nums, find the subarray with the largest sum, and return its sum.
*/

// Approach:
// We use Kadane's Algorithm to find the max continuous sum in an array
// 1) Init max_sum to first element and cont_sum to 0
// 2) Loop through nums
// 3) add each element to cont_sum
// 4) if cont_sum is > max sum we set max_sum = cont_sum
// 5) discard cont_sum if it is negetive
// 6) return max_sum

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max_sum = nums[0];
        let mut cont_sum = 0;
        for n in &nums {
            cont_sum += n;
            if cont_sum > max_sum {max_sum = cont_sum;}
            if cont_sum < 0 {cont_sum = 0;}
        }
        max_sum
    }
}
