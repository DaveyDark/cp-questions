/* 
Question:
Given a sorted array of distinct integers and a target value, return the index if the target is found. 
If not, return the index where it would be if it were inserted in order.
You must write an algorithm with O(log n) runtime complexity.
*/

// Approach:
// 1) We perform normal binary seach in the given vector and return the target if found
// 2) Otherwise we return mid+1 if the mid is less than the target
//    or mid if the mid is greater

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut left: i32 = 0;
        let mut right: i32 = nums.len() as i32 -1;
        let mut mid: i32 = 0;
        loop {
            mid = left + (right-left)/2;
            if nums[mid as usize] == target  {
                return mid;
            }
            else if target < nums[mid as usize] {
                right = mid-1;
            } else {
                left = mid+1;
            }
            if(left > right) {break;}
        }
        if nums[mid as usize] < target {
            mid+1
        } else {
            mid
        }
    }
}
