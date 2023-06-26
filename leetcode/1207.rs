/*
Question:
Given an array of integers arr, return true if the number of occurrences of each value in the array is unique or false otherwise.
*/

// Approach:
// 1) We first make a HashMap and then record the frequencies of the elements of the vector in it
// 2) Then we make a HashSet and go over the values of the map
//    If the entry for a value already exists we return false
//    Otherwise we insert it into the set
// 3) At the end we return false

impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let mut map = std::collections::HashMap::new();
        for n in arr.iter() {
            *map.entry(*n).or_insert(0) += 1;
        }
        let mut seen = std::collections::HashSet::new();
        for val in map.values() {
            match seen.get(val) {
                Some(_) => { return false; }
                None => { seen.insert(val); }
            }
        }
        true
    }
}
