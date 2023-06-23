/*
Question:
You are given an integer array height of length n. There are n vertical lines drawn such that the two endpoints of the ith line are (i, 0) and (i, height[i]).
Find two lines that together with the x-axis form a container, such that the container contains the most water.
Return the maximum amount of water a container can store.
Notice that you may not slant the container.
*/

// Approach:
// 1) We make two pointers for the left and right of the array
// 2) Then we loop while left < right
//    Each iteration we calculate area between the left and right index by multiplying the lesser of the two heights with the distance between then,
//    and we set area_max to that value if it is greater
//    Then we move the shorter of the left and right heights towards the center
// 3) At the end we return the calculated max area

class Solution {
    public int maxArea(int[] height) {
        int left = 0, right = height.length - 1, area_max = 0;
        while(left<right){
            area_max = Math.max(area_max, Math.min(height[left],height[right]) * (right-left));
            if(height[left]>height[right]) right--;
            else left++;
        }
        return area_max;
    }
}
