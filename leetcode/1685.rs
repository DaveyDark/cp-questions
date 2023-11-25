/*
Question:
You are given an integer array nums sorted in non-decreasing order.

Build and return an integer array result with the same length as nums 
such that result[i] is equal to the summation of absolute differences 
between nums[i] and all the other elements in the array.

In other words, result[i] is equal to sum(|nums[i]-nums[j]|) where 0 <= j < nums.length and j != i (0-indexed).
*/

// Appraoch:
// 1) Make a vector to store the solution array of sums and variables to store the prefix and
//    suffix sums
// 2) Loop over the nums array and each iteration, update suffix and prefix sums and push the
//    current sum to sums calculated using the formula:
//      ((nums[i] * i) - prefix[i]) + (suffix[i] - (nums[i] * (n-i-1)));
// 3) Return the sums array

impl Solution {
    pub fn get_sum_absolute_differences(nums: Vec<i32>) -> Vec<i32> {
        let mut sums = vec!();
        let mut prefix = 0;
        let mut suffix: i32 = nums.iter().sum();
        let l = nums.len() as i32;
        for (i,n) in nums.iter().enumerate() {
            suffix -= n;
            sums.push(((n * i as i32) - prefix) + (suffix - (n * (l - i as i32 - 1))));
            prefix += n;
        }
        sums
    }
}
