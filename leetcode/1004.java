/*
Question:
Given a binary array nums and an integer k, return the maximum number of consecutive 1's in the array if you can flip at most k 0's.
*/

// Approach
// 1) We make some variables, streak to track the consecutive ones in the current window, max tracks the max streak, 
//    errors tracks the number fo 0s flipped in window, and start tracks start of the window
// 2) We loop through the array, and each iteration increase streak if the current number is 1
//    Otherwise if it's 0 but errors < k i.e. we can flip more 0s, we do so
//    Otherwise we move the start of the array forward until we have removed a 0, and decrease streak accordingly
//    At teh end of the iteration we compare max with streak to make it store the greater value
// 3) At the end of the loop we return max

class Solution {
    public int longestOnes(int[] nums, int k) {
        int streak = 0, errors = 0, start = 0, max = 0;
        for(int i=0; i<nums.length; i++) {
            if(nums[i] == 1)streak++;
            else {
                if(errors < k){
                    errors++;
                    streak++;
                } else {
                    while(nums[start++] != 0){ streak--; }
                }
            }
            max = Math.max(max,streak);
        }
        return max;
    }
}
