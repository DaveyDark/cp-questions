/*
Question:
You are given an integer array nums. You need to create a 2D array from nums satisfying the following conditions:
    The 2D array should contain only the elements of the array nums.
    Each row in the 2D array contains distinct integers.
    The number of rows in the 2D array should be minimal.

Return the resulting array. If there are multiple answers, return any of them.

Note that the 2D array can have a different number of elements on each row.
*/

// Approach
// 1) Make a frequency array and a 2D arraylist
// 2) Iterate through the array and for each element:
//    Increment the frequency of the element
//    If the size of the 2D arraylist is less than the frequency of the element, add a new arraylist
//    Add the element to the arraylist at the index of the frequency of the element minus 1
// 3) Return the 2D arraylist

import java.util.*;

class Solution {
    public List<List<Integer>> findMatrix(int[] nums) {
        int[] freq = new int[nums.length+1];
        List<List<Integer>> res = new ArrayList<>();
        for(int num: nums) {
            freq[num] += 1;
            if(res.size() < freq[num]) res.add(new ArrayList<Integer>());
            res.get(freq[num]-1).add(num);
        }
        return res;
    }
}
