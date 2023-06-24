/*
Question:
You are given an integer array nums and an integer k.
In one operation, you can pick two numbers from the array whose sum equals k and remove them from the array.
Return the maximum number of operations you can perform on the array.
*/

// Approach:
// 1) We make two pointers for the left and right of the array, and a variable to track the number of operations that can be performed
// 2) Then we sort the array
// 3) Then we loop while left<right
//    Each iteration we calculate the sum of elements at the two pointers
//    If the sum is equal to k, we can perform an operation so we increment op
//    If the sum is <= k, the sum is less than needed so we need to increase one of the numbers which can be done be incrementing left pointer
//    If the sum is >= k, the sum is more than required so we need to decrease one of the numbers which can be done by decrementing right pointer
//    When the sum is equal, both pointers are moved because that pair of number is not unusable so we need to move the pointers away
// 4) We return the numer of operations performed(op) at the end of the loop

class Solution {
    public int maxOperations(int[] nums, int k) {
        int left = 0, right = nums.length-1;
        int op = 0;
        Arrays.sort(nums);
        while(left<right) {
            int sum = nums[left] + nums[right];
            if(sum == k)op++;
            if(sum <= k)left++;
            if(sum >= k)right--;
        }
        return op;
    }
}
