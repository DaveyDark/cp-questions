/*
Question:
You are given a sorted unique integer array nums.
A range [a,b] is the set of all integers from a to b (inclusive).
Return the smallest sorted list of ranges that cover all the numbers in the array exactly. 
That is, each element of nums is covered by exactly one of the ranges, and there is no integer x such that x is in one of the ranges but not in nums.
Each range [a,b] in the list should be output as:
    "a->b" if a != b
    "a" if a == b
*/

// Approach:
// 1) We first make an ArrayList of strings for the solution, along with ints to keep track of stand and end of current range
// 2) Then we check the edge case that there is only 1 number in array, and return that number as a string if that is the case
// 3) Then we go over the nums array and check each element
//    If current element is 1 more than the last(range end) element, we increment range end since the range is continuing
//    We also check if it si the last iteration and end the range if it is
//    Otherwise, we check if the range only has a single element(start==end), and push that single number if that's the case
//    Otherwise, we add the range from start index to end index
//    Then we reset the range
//    We also check it is the the last iteration here and push the current element if it is
// 4) We return the ArrayList at the end

class Solution {
    public List<String> summaryRanges(int[] nums) {
        int start = 0, end = 0; // start and end of current range
        ArrayList<String> ans = new ArrayList<String>();
        if (nums.length == 1){
            ans.add(String.valueOf(nums[0]));
            return ans;
        }
        for(int i = 1; i < nums.length; i++) {
            if (nums[i] == nums[end]+1) {
                end++;
                if (i == nums.length-1) ans.add(nums[start] + "->" + nums[i]);
            }
            else {
                if (start-end != 0) {
                    ans.add(nums[start] + "->" + nums[end]);
                } else {
                    ans.add(String.valueOf(nums[start]));
                }
                start = i;
                end = i;
                if (i == nums.length-1) ans.add(String.valueOf(nums[i]));
            }
        }
        return ans;
    }
}
