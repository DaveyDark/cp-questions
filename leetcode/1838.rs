/*
Question:
The frequency of an element is the number of times it occurs in an array.

You are given an integer array nums and an integer k. 
In one operation, you can choose an index of nums and increment the element at that index by 1.

Return the maximum possible frequency of an element after performing at most k operations.
*/

// Approach:
// 1) Sort the array
// 2) Make variabes to track the max frequency recorded so far, the start of the window and the sum
//    of elements in the window
// 3) Loop over the array making the current index the right edge of the window
//      Each iteraion, add the new element to the sum and then loop while difference of size of the window into the
//      current element and the sum of the window is more than k( (right-left+1) * nums[right] - sum )
//      and shrink the window by subracting the leftmost element from the sum and incrementing left
//      Then at the end of iteration we set the max_cnt to the max of the current window size
// 4) Return the max_cnt

impl Solution {
    pub fn max_frequency(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();
        let mut max_cnt = 0;
        let mut left = 0;
        let mut sum = 0;
        for right in 0..nums.len() {
            sum += nums[right];
            while (right-left+1) as i32 * nums[right] - sum > k {
                sum -= nums[left];
                left += 1;
            }
            max_cnt = max_cnt.max(right - left + 1);
        }
        max_cnt as i32
    }
}
