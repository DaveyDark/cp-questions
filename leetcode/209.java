/*
Question:
Given an array of positive integers nums and a positive integer target, return the minimal length of a subarray
whose sum is greater than or equal to target. If there is no such subarray, return 0 instead.
*/

// Approach:
// 1) We make variables to track the start and sum of the sliding window, and min to store the minimum length
//    min is initialized to length of nums as it is the largest possible length of a subarray 
// 2) Then we go through the array, and add each number to sum
//    If the sum is more than the target, we remove all the elements from the window we can without making the sum too small
//    Then we see if the length of the window is less then the minimum recorded length, and set min to it if it is
// 3) At the end, if the sum is still less than target then we return 0 because the sum of the whole array is still inadequate
//    Otherwise we return min

class Solution {
    public int minSubArrayLen(int target, int[] nums) {
        int sum = 0, start = 0, min = nums.length;
        for(int i=0; i<nums.length; i++){
            sum += nums[i];
            if(sum >= target){
                while(sum-nums[start] >= target){
                    sum-=nums[start];
                    start++;
                }
                if((i-start) + 1 < min) min = (i-start)+1;
            }
        }
        if(sum<target && min==nums.length)return 0;
        return min;
    }
}
