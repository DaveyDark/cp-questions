/*
Question:
You are given two 0-indexed integer arrays nums1 and nums2 of equal length n and a positive integer k.
You must choose a subsequence of indices from nums1 of length k.

For chosen indices i0, i1, ..., ik - 1, your score is defined as:
    The sum of the selected elements from nums1 multiplied with the minimum of the selected elements from nums2.
    It can defined simply as: (nums1[i0] + nums1[i1] +...+ nums1[ik - 1]) * min(nums2[i0] , nums2[i1], ... ,nums2[ik - 1]).

Return the maximum possible score.

A subsequence of indices of an array is a set that can be derived from the set {0, 1, ..., n-1} by deleting some or no elements.
*/

// Approach:
// 1) Combine nums1 and nums2 into a vector of pairs (num1, num2)
// 2) Sort the vector by num2 in descending order
// 3) Create a max heap and variable to track the max score and running sum
// 4) Iterate through the vector
//    Add the current num1 to the heap and sum
//    If the heap is full, calculate the score and pop the heap and update sum
// 5) Return the max score

use std::collections::BinaryHeap;

impl Solution {
    pub fn max_score(mut nums1: Vec<i32>, mut nums2: Vec<i32>, k: i32) -> i64 {
        let mut nums = nums1
            .into_iter()
            .zip(nums2.into_iter())
            .collect::<Vec<(i32, i32)>>();
        nums.sort_unstable_by_key(|n| -n.1);
        let mut heap: BinaryHeap<i32> = BinaryHeap::new();
        let mut score = 0;
        let mut sum = 0;
        for i in 0..nums.len() {
            heap.push(-nums[i].0);
            sum += nums[i].0 as i64;
            if heap.len() == k as usize {
                score = score.max(nums[i].1 as i64 * sum);
                sum += heap.pop().unwrap() as i64;
            }
        }
        score
    }
}
