/*
Question:
There is an integer array nums sorted in ascending order (with distinct values).

Prior to being passed to your function, nums is possibly rotated at an unknown pivot index k (1 <= k < nums.length) 
such that the resulting array is [nums[k], nums[k+1], ..., nums[n-1], nums[0], nums[1], ..., nums[k-1]] (0-indexed). 
For example, [0,1,2,4,5,6,7] might be rotated at pivot index 3 and become [4,5,6,7,0,1,2].

Given the array nums after the possible rotation and an integer target, return the index of target if it is in nums, or -1 if it is not in nums.

You must write an algorithm with O(log n) runtime complexity.
*/

// Approach:
// At any point in a sorted rotated arrray, on side of the point will be properly sorted while the other part will have the pivot
// 1) Make variables to store the left and right limits of the binary search
// 2) loop while left <= right and calculate mid each iteration
//      We can directly return mid if it matches the target
//      Otherwise we check which of the two sides have the sorted half
//      If the left side has the sorted half, we check if the target is inside the sorted range and set right if it is, otherwise we set left
//      Vice Versa for the right side
// 3) Outside the loop, return -1 since the element wasn't found

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut left, mut right) = (0,nums.len() as i32 - 1);
        while left <= right {
            let mid = left + (right-left)/2;
            if nums[mid as usize] == target {
                return mid;
            } else if nums[left as usize] <= nums[mid as usize] {
                if target < nums[mid as usize] && target >= nums[left as usize] {
                    right = mid-1;
                } else {
                    left = mid+1;
                }
            } else {
                if target > nums[mid as usize] && target <= nums[right as usize]{
                    left = mid+1;
                } else {
                    right = mid-1;
                }
            }
        }
        -1
    }
}
