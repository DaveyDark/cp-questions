/*
Question:
The product difference between two pairs (a, b) and (c, d) is defined as (a * b) - (c * d).
    For example, the product difference between (5, 6) and (2, 7) is (5 * 6) - (2 * 7) = 16.

Given an integer array nums, choose four distinct indices w, x, y,
and z such that the product difference between pairs (nums[w], nums[x])
and (nums[y], nums[z]) is maximized.

Return the maximum such product difference.
*/

// Approach:
// 1) Create 4 variables to store the largest and smallest 2 numbers.
// 2) Iterate over the array and update the variables accordingly.
// 3) Return the difference of the product of the largest and smallest 2 numbers.

impl Solution {
    pub fn max_product_difference(nums: Vec<i32>) -> i32 {
        let mut smallest1 = i32::MAX;
        let mut smallest2 = i32::MAX;
        let mut largest1 = i32::MIN;
        let mut largest2 = i32::MIN;

        for &n in &nums {
            if n > largest2 {
                if n > largest1 {
                    largest2 = largest1;
                    largest1 = n;
                } else {
                    largest2 = n;
                }
            }
            if n < smallest2 {
                if n < smallest1 {
                    smallest2 = smallest1;
                    smallest1 = n;
                } else {
                    smallest2 = n;
                }
            }
        }

        (largest2 * largest1) - (smallest1 * smallest2)
    }
}
