/*
Question:
Given an integer array nums, your goal is to make all elements in nums equal. To complete one operation, follow these steps:
    Find the largest value in nums. Let its index be i (0-indexed) and its value be largest. 
If there are multiple elements with the largest value, pick the smallest i.
    Find the next largest value in nums strictly smaller than largest. Let its value be nextLargest.
    Reduce nums[i] to nextLargest.

Return the number of operations to make all elements in nums equal.
*/

// Approach:
// 1) Sort the given array
// 2) Make a counter to count the operations and also a variable to track the last element,
//    initialized to the last element of the array
// 3) Loop over the array starting from the second last index
//      If the current element is different from the last, we add the number of elements to the
//      right of i to the ops
//      And at the end of iteration set last to the current element
// 4) Return the ops counter

impl Solution {
    pub fn reduction_operations(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let mut ops = 0;
        let mut last = nums[nums.len()-1];
        for i in (0..nums.len()-1).rev() {
            if nums[i] != last {
                ops += (nums.len() - 1 - i);
            }
            last = nums[i];
        }
        ops as i32
    }
}
