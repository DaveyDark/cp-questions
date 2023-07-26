/*
Question:
You are given two 2D integer arrays nums1 and nums2.

    nums1[i] = [idi, vali] indicate that the number with the id idi has a value equal to vali.
    nums2[i] = [idi, vali] indicate that the number with the id idi has a value equal to vali.

Each array contains unique ids and is sorted in ascending order by id.

Merge the two arrays into one array that is sorted in ascending order by id, respecting the following conditions:

    Only ids that appear in at least one of the two arrays should be included in the resulting array.
    Each id should be included only once and its value should be the sum of the values of this id in the two arrays. 
    If the id does not exist in one of the two arrays then its value in that array is considered to be 0.

Return the resulting array. The returned array must be sorted in ascending order by id.
*/

// Approach:
// 1) Make a hashmap and vector
// 2) Add the elements of nums1 and nums2 to the hashmap, and sum them if they already exist in the map
// 3) Add the elements of the hashmap to the vector
// 4) Sort the vector using x[0] as the key
// 5) Return the vector

use std::collections::HashMap;

impl Solution {
    pub fn merge_arrays(nums1: Vec<Vec<i32>>, nums2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut map = HashMap::new();
        let mut arr = Vec::new();
        for num in nums1 {
            *map.entry(num[0]).or_insert(0) += num[1];
        }
        for num in nums2 {
            *map.entry(num[0]).or_insert(0) += num[1];
        }
        for (k,v) in map {
            arr.push(vec!(k,v));
        }
        arr.sort_by_key(|x| x[0]);
        arr
    }
}
