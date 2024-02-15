/*
Question:
You are given an array of positive integers nums of length n.

A polygon is a closed plane figure that has at least 3 sides. The longest side of a polygon is smaller than the sum of its other sides.

Conversely, if you have k (k >= 3) positive real numbers a1, a2, a3, ..., ak 
where a1 <= a2 <= a3 <= ... <= ak and a1 + a2 + a3 + ... + ak-1 > ak, 
then there always exists a polygon with k sides whose lengths are a1, a2, a3, ..., ak.

The perimeter of a polygon is the sum of lengths of its sides.

Return the largest possible perimeter of a polygon whose sides can be formed from nums, or -1 if it is not possible to create a polygon.
*/

// Approach:
// 1) Sort the array
// 2) Make a sum variable and initialize it with the sum of first two elements. Initialize perimeter to -1
// 3) Iterate from the third element to the end of the array
//    Each time, check if the sum is greater than the current element 
//     and set perimeter to the maximum of the current perimeter and the sum + current element if it is
//    Add the current element to the sum
// 4) Return the perimeter

class Solution {
  public long largestPerimeter(int[] nums) {
    Arrays.sort(nums);
    long sum = nums[0] + nums[1];
    long pm = -1;
    for(int i = 2; i < nums.length; i++) {
      if(sum > nums[i]) pm = Math.max(pm, sum + nums[i]);
      sum += nums[i];
    }
    return pm;
  }
}
