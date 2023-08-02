/*
Queestion:
Given an array nums of distinct integers, return all the possible permutations. You can return the answer in any order.
*/

// Approach:
// 1) Make a 2d List of Integers to store the permutations
// 2) Make and use generate() to generate the permutations
//    generate() will take an additional parameter called fixed, which is the index of last fixed element in the current permutation
//    If all numbers are fixed, i.e. fixed == nums.length-1, then add the current order of nums to permutations
//    Otherwise we loop over all non-fixed indices and fix each one of them one by one 
//    We do this by swapping them with the fixed index and calling generate to generate combinations with that element fixed
// 3) After the call to generate(), permutations will be updated and we can return it

class Solution {
    void generate(int[] nums, List<List<Integer>> permutations, int fixed) {
        if(fixed == nums.length-1) {
            List<Integer> list = new ArrayList<>();
            for(int num: nums) list.add(num);
            permutations.add(list);
            return;
        }
        for(int i=fixed; i<nums.length; i++) {
            int tmp = nums[fixed];
            nums[fixed] = nums[i];
            nums[i] = tmp;
            this.generate(nums,permutations,fixed+1);
            nums[i] = nums[fixed];
            nums[fixed] = tmp;
        }
    }
    public List<List<Integer>> permute(int[] nums) {
        List<List<Integer>> permutations = new ArrayList<List<Integer>>();
        this.generate(nums,permutations,0);
        return permutations;
    }
}
