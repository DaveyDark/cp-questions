/*
Question:
Given two sorted arrays nums1 and nums2 of size m and n respectively, return the median of the two sorted arrays.

The overall run time complexity should be O(log (m+n)).
*/

// Aproach:
// 1) Append the nums2 to nums1
// 2) Sort nums1
// 3) If length of nums1 is even, we return the average of n/2 and n/2-1 indices of nums1
//    Otherwise we return the element at index n/2

impl Solution {
    pub fn find_median_sorted_arrays(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> f64 {
        nums1.append(&mut nums2);
        nums1.sort_unstable();
        if nums1.len()%2 == 0 {
            (nums1[nums1.len()/2] as f64 + nums1[nums1.len()/2 -1] as f64)/2.0
        } else {
            nums1[nums1.len()/2] as f64
        }
    }
}
