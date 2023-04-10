/*
Question:
You are given two integer arrays nums1 and nums2, sorted in non-decreasing order, 
and two integers m and n, representing the number of elements in nums1 and nums2 respectively.
Merge nums1 and nums2 into a single array sorted in non-decreasing order.
The final sorted array should not be returned by the function, but instead be stored inside the array nums1. 
To accommodate this, nums1 has a length of m + n, where the first m elements denote the elements that should be merged, 
and the last n elements are set to 0 and should be ignored. nums2 has a length of n.
*/

// Approach:
// 1) Truncate the nums1 vector to remove any elements after the first m elements.
// 2) Initialize two indices: nums1_idx for nums1 and nums2_idx for nums2.
// 3) Loop over nums2:
// 4) Check if there is an element at the current index of nums1.
//    If there is an element, compare it to the current element of nums2.
//    If the current element of nums2 is less than the current element of nums1, insert the current element of nums2 at the current index of nums1, increment nums2_idx, and continue the loop.
//    If the current element of nums2 is greater than or equal to the current element of nums1, increment nums1_idx and continue the loop.
// 5) If there is no element at the current index of nums1, append the remaining elements of nums2.

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        //remove any elements after the first `m` elements.
        nums1.truncate(m as usize);

        // Initialize indices for `nums1` and `nums2`.
        let mut nums1_idx = 0;
        let mut nums2_idx = 0;

        // Loop over `nums2`
        while nums2_idx < n as usize {
            // Check if there is an element at the current index of `nums1`
            match nums1.get(nums1_idx) {
                // If there is an element, compare it to the current element of `nums2`
                Some(&num1) => {
                    if nums2[nums2_idx] < num1 {
                        // If the current element of `nums2` is less than the current element of `nums1`
                        // insert the current element of `nums2` at the current index of `nums1`.
                        nums1.insert(nums1_idx, nums2[nums2_idx]);
                        nums2_idx += 1;
                    } else {
                        // If the current element of `nums2` is greater than or equal to the current element
                        // of `nums1`, move to the next index of `nums1`.
                        nums1_idx += 1;
                    }
                }
                // If there is no element at the current index of `nums1`, append the remaining elements of `nums2`
                None => {
                    nums1.extend(nums2.iter().skip(nums2_idx));
                    break;
                }
            }
        }
    }
}
