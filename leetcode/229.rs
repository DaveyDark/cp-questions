/*
Question:
Given an integer array of size n, find all elements that appear more than ⌊ n/3 ⌋ times.
*/

// Approach:
// 1) Make a hashmap to store the frequencies of elements
// 2) Loop over given elements and record their frequencies in the map
// 3) Filter the map to only have elements with > l/3 frequencies, then collect and 
//    return the keys of remaining elements into a vector

use std::collections::HashMap;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        let l = nums.len();
        let mut map = HashMap::new();
        nums.iter().for_each(|x| *map.entry(x).or_insert(0) += 1);
        map.iter().filter(|(k,v)| **v > l/3).map(|(k,v)| **k).collect()
    }
}
