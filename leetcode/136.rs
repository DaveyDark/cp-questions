/*
Question:
Given a non-empty array of integers nums, every element appears twice except for one. Find that single one.
You must implement a solution with a linear runtime complexity and use only constant extra space.
*/

// Approach:
// If we XOR any number with the same element twice, it has no effect. Therefore we jsut XOR the whole vector together and the duplicated cancel each other out and we only have the bits of the unique number left
// To do this, We go through the array using fold, which goes through an iterator keeping a  acculuted value, with an accumulated value acc and update it by XORing it with the current element.

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        nums.iter().fold(0,|ans,x| ans ^ x)
    }
}
