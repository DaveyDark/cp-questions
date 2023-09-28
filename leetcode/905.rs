/*
Question:
Given an integer array nums, move all the even integers at the beginning of the array followed by all the odd integers.

Return any array that satisfies this condition.
*/

// Approach:
// 1) Make a pointer for the start of the array
// 2) Loop through the given array
// 3) If the number is even ,we swap it with the number at the pointer and increment the pointer 
// 4) Return the modified array

impl Solution {
    pub fn sort_array_by_parity(mut nums: Vec<i32>) -> Vec<i32> {
        let mut pnt = 0;
        for i in 0..nums.len() {
            if nums[i]%2==0 {
                nums.swap(pnt,i);
                pnt += 1;
            }
        }
        nums
    }
}
