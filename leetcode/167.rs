/*
Question:
Given a 1-indexed array of integers numbers that is already sorted in non-decreasing order, find two numbers such that they add up to a specific target number. 
Let these two numbers be numbers[index1] and numbers[index2] where 1 <= index1 < index2 <= numbers.length.
Return the indices of the two numbers, index1 and index2, added by one as an integer array [index1, index2] of length 2.
The tests are generated such that there is exactly one solution. You may not use the same element twice.
Your solution must use only constant extra space.
*/

// Approach:
// 1) We keep variables to rack the left and right indexes and loop until they are unequal
// 2) If the sum of numbers at left and right indices is the target, we return the indices
// 3) If it's less than target, we increment the left index
// 4) Otherwise we decrement the right index

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut left = 0;
        let mut right = numbers.len() as i32 - 1;
        let mut x;
        while left < right {
            x = numbers[left as usize] + numbers[right as usize];
            if(x == target) { return vec!(left+1, right+1); }
            else if (x < target) {left += 1;}
            else {right -= 1;}
        }
        vec!()
    }
}
