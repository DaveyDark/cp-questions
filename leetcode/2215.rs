/*
Question:
Given two 0-indexed integer arrays nums1 and nums2, return a list answer of size 2 where:
    answer[0] is a list of all distinct integers in nums1 which are not present in nums2.
    answer[1] is a list of all distinct integers in nums2 which are not present in nums1.
Note that the integers in the lists may be returned in any order.
*/

// Approach:
// 1) We make sets from the the given vectors which eliminates duplicates
// 2) Then we vectorize using vec! the difference of the first set with the second and vice versa and return it\

use std::collections::HashSet;

impl Solution {
    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        let set1: HashSet<i32> = nums1.iter().copied().collect();
        let set2: HashSet<i32> = nums2.iter().copied().collect();

        vec![
            set1.difference(&set2).copied().collect(),
            set2.difference(&set1).copied().collect(),
        ]
    }
}
