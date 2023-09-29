/*
Question:
An array is monotonic if it is either monotone increasing or monotone decreasing.
An array nums is monotone increasing if for all i <= j, nums[i] <= nums[j]. An array nums is monotone decreasing if for all i <= j, nums[i] >= nums[j].
Given an integer array nums, return true if the given array is monotonic, or false otherwise.
*/

// Approach:
// 1) If the array has less than 2 elements, return true
// 2) Create a variable delta and set to tot he difference of the first and second element
// 3) Loop through the rest of the elements in the array
//      If delta is negative and the current element is less than the last, return false
//      Otherwise if delta is positive and current element is more than the last, return false
//      Otherwise if delta is still 0, set it to the difference of the last and current element
// 4) Return true

class Solution {
    public boolean isMonotonic(int[] nums) {
        if(nums.length < 2) return true;
        int delta = nums[0] - nums[1];
        for(int i=1; i<nums.length; i++) {
            if(delta < 0 && nums[i] < nums[i-1])return false;
            else if(delta > 0 && nums[i] > nums[i-1])return false;
            else if(delta == 0) delta = nums[i-1] - nums[i];
        }
        return true;
    }
}
