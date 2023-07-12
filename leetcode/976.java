/*
Question:
Given an integer array nums, return the largest perimeter of a triangle with a non-zero area, formed from three of these lengths. If it is impossible to form any triangle of a non-zero area, return 0.
*/

// Approach
// If the side of a triangle are a,b,c then a+b>c for the triangle to be formed. If we know c is the largest, we only need a+b>c to statisfy
// 1) Sort the array to get the maximum elements of the array towards the end
// 2) Traverse the array bacckward to find nums[i]<nums[i-1]+nums[i-2] till the index 2, and return the sum if such a pair exists
// 3) Otherwise return 0 as the default value outside the loop

class Solution {
    public int largestPerimeter(int[] nums) {
        Arrays.sort(nums);
        for(int i=nums.length-1; i>1; i--) {
            if(nums[i-1] + nums[i-2] > nums[i]) {
                return nums[i] + nums[i-1] + nums[i-2];
            }
        }
        return 0;
    }
}
