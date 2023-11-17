/*
Question:
The pair sum of a pair (a,b) is equal to a + b. The maximum pair sum is the largest pair sum in a list of pairs.
    For example, if we have pairs (1,5), (2,3), and (4,4), the maximum pair sum would be max(1+5, 2+3, 4+4) = max(6, 5, 8) = 8.

Given an array nums of even length n, pair up the elements of nums into n / 2 pairs such that:
    Each element of nums is in exactly one pair, and
    The maximum pair sum is minimized.

Return the minimized maximum pair sum after optimally pairing up the elements.
*/

// Approach:
// 1) Sort the array
// 2) Make variable to track the max sum
// 3) Loop over indices from 0 to n/2 and make pairs of elements with index i and n-1-i
//    And update max if needed for pair
// 4) Return max

impl Solution {
    pub fn min_pair_sum(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let mut max = 0;
        for i in 0..nums.len()/2 {
            max = max.max(nums[i] + nums[nums.len() - 1 - i]);
        }
        max
    }
}
