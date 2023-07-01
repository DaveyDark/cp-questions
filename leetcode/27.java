/*
Question:
Given an integer array nums and an integer val, remove all occurrences of val in nums in-place. The order of the elements may be changed. Then return the number of elements in nums which are not equal to val.
Consider the number of elements in nums which are not equal to val be k, to get accepted, you need to do the following things:
    Change the array nums such that the first k elements of nums contain the elements which are not equal to val. The remaining elements of nums are not important as well as the size of nums.
    Return k.
Custom Judge:
The judge will test your solution with the following code:
int[] nums = [...]; // Input array
int val = ...; // Value to remove
int[] expectedNums = [...]; // The expected answer with correct length.
                            // It is sorted with no values equaling val.
int k = removeElement(nums, val); // Calls your implementation
assert k == expectedNums.length;
sort(nums, 0, k); // Sort the first k elements of nums
for (int i = 0; i < actualLength; i++) {
    assert nums[i] == expectedNums[i];
}
If all assertions pass, then your solution will be accepted.
*/

// Approach:
// 1) We make a variable to track number of unequal elements, and a variable to store the pointer to the insertion location
// 2) Then we go over the array and check if the current element is equal to val
//    If it is, we swap the elements at i and ptr indices, and decrease ptr
//    Otherwise we increment cnt
// 3) At the end we return cnt, which is the size of the new array

class Solution {
    public int removeElement(int[] nums, int val) {
        int cnt = 0;
        int ptr = nums.length-1;
        for(int i = nums.length-1; i >= 0; i--) {
            if (nums[i] == val) {
                int tmp = nums[i];
                nums[i] = nums[ptr];
                nums[ptr] = tmp;
                ptr--;
            } else cnt++;
        }
        return cnt;
    }
}
