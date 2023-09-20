/*
Question:
You are given an integer array nums and an integer x. 
In one operation, you can either remove the leftmost or the rightmost element from the array nums and subtract its value from x. 
Note that this modifies the array for future operations.

Return the minimum number of operations to reduce x to exactly 0 if it is possible, otherwise, return -1.
*/

// Approach:
// We use a sliding window and move thorugh the array while keeping track of the sum of the
// remaining array
// 1) Init sum to the sum of the array and start and end pointers to 0, and also ops to None
// 2) Loop until the window reaches the end of the array and the sum exceeds x and the window can't
//    contract further
// 3) If the sum is less than x and the window can shrink, shrink the window
//    Oterwise expand the window
// 4) If sum is equal to x, set ops to the min of it's previous value and the current number of ops
//    needed. Use i32::MAX if ops isn't set yet
// 5) Return ops and if it isn't set then return -1 instead

impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        let mut sum: i32 = nums.iter().sum();
        let (mut start, mut end) = (0,0);
        let mut ops = None;
        while end < nums.len() || (sum < x && start < end) {
            if sum < x && start < end {
                sum += nums[start];
                start += 1;
            } else {
                sum -= nums[end];
                end += 1;
            }
            if sum == x {
                ops = Some(ops.unwrap_or(i32::MAX).min(nums.len() as i32 - (end - start) as i32));
            }
        }
        ops.unwrap_or(-1)
    }
}
