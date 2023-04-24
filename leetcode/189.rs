/*
Question:
Given an integer array nums, rotate the array to the right by k steps, where k is non-negative.
*/

// Approach:
// 1) We reduce k to k % nums.len()
// 2) We return of k is 0
// 3) We loop k times and rotate the vector by 1 each time by popping the last element and inserting it at the front

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let k = k as usize % nums.len();
        if k <= 0 { return; }
        let mut x;
        for i in 0..k{
            x = nums.pop().unwrap();
            nums.insert(0,x);
        }
    }
}
