/*
Question:
Given n non-negative integers representing an elevation map where the width of each bar is 1, compute how much water it can trap after raining.
*/

// Approach:
// We use two pointers approach, and move the lesser pointer towards the center while keeping track of the left and right maximums
// 1) Init the two pointer to the left and right edge of the array, and the left and right max to the leftmost and rightmost element
// 2) Loop intil left pointer doesn't meet right pointer, and each iteration check:
//    If the left maximum is lesser, then we move the left pointer towards the middle
//      Then check if the current element at left pointer is more than left max. and set leftmax to it if it is
//      Otherwise add leftmax - height[keft] to the water collected 
//      (This is because if there is a higher wall of the left, then water will collect upto that height)
//    Otherwise if the rightmax is lesser, we move the right pointer towards teh middle
//      Then similarly check and set rightmax if it is lesser
//      Otherwise add rightmax - height[right] to water
// 3) Return water at the end

class Solution {
    public int trap(int[] height) {
        int water = 0;
        int left=0,right=height.length-1;
        int leftMax = height[0], rightMax = height[height.length - 1];

        while(left<right) {
            if (leftMax < rightMax) {
                left++;
                if (leftMax < height[left]) {
                    leftMax = height[left];
                } else {
                    water += leftMax - height[left];
                }
            } else {
                right--;
                if (rightMax < height[right]) {
                    rightMax = height[right];
                } else {
                    water += rightMax - height[right];
                }
            }
        }

        return water;
    }
}
