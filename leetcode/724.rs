/*
Question:
Given an array of integers nums, calculate the pivot index of this array.
The pivot index is the index where the sum of all the numbers strictly to the left of the index is equal to the sum of all the numbers strictly to the index's right.
If the index is on the left edge of the array, then the left sum is 0 because there are no elements to the left. This also applies to the right edge of the array.
Return the leftmost pivot index. If no such index exists, return -1.
*/

// Approach:
// 1) We make two variables to keep track of the current sum on the left and right side, and initialize right with the sum of nums excluding the first element
// 2) Then we go over nums and each iteration we check if left sum = right sum and return i if it is
//    Otherwise we update left sum by adding the current element and right sum by subtracting the next element
// 3) If we reach out of the loop then we return -1

impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let mut left_sum: i32 = 0;
        let mut right_sum: i32 = nums[1..].iter().sum();
        for i in 0..nums.len() {
            if left_sum == right_sum {return i as i32}
            left_sum += nums[i];
            right_sum -= *nums.get(i+1).unwrap_or(&0);
        }
        -1
    }
}
