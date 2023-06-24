/*
Question:
Given a binary array nums, you should delete one element from it.
Return the size of the longest non-empty subarray containing only 1's in the resulting array. Return 0 if there is no such subarray.
*/

// Approach:
// 1) We mave variables to store the start of the sliding window, the max size and a flag to indicate if we have deleted an element yet
// 2) Then we loop through the array, and if the curretn element is 0 then we flag delete if it isn't flagged yet
//    Otherwise we move start to (the index of the next 0)+1
//    At the end of the iteration we set max to the greater of max recorded size and the current size of the window
// 3) Outside the loop we return max

class Solution {
    public int longestSubarray(int[] nums) {
        int start = 0, max = 0;
        boolean deleted = false;
        for(int i=0; i<nums.length; i++) {
            if(nums[i] == 0) {
                if(!deleted)deleted = true;
                else while(nums[start++] != 0);
            }
            max = Math.max(max, i-start);
        }
        return max;
    }
}
