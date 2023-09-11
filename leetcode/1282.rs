/*
Question:
There are n people that are split into some unknown number of groups. Each person is labeled with a unique ID from 0 to n - 1.

You are given an integer array groupSizes, where groupSizes[i] is the size of the group that person i is in. 
For example, if groupSizes[1] = 3, then person 1 must be in a group of size 3.

Return a list of groups such that each person i is in a group of size groupSizes[i].

Each person should appear in exactly one group, and every person must be in a group. 
If there are multiple answers, return any of them. It is guaranteed that there will be at least one valid solution for the given input.
*/

// Approach:
// 1) Make a hashmap to store the vectors for each group size and another vector to store the final
//    groups
// 2) Loop through the given sizes and push each element to the respective group size
//      If the vector for a certian size is full, we push it to groups and clear it
// 3) Return the final groups vector

use std::collections::HashMap;

impl Solution {
    pub fn group_the_people(group_sizes: Vec<i32>) -> Vec<Vec<i32>> {
        let mut group_map: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut groups = vec!();
        for i in 0..group_sizes.len() {
            group_map.entry(group_sizes[i]).or_insert(vec!()).push(i as i32);
            if group_map.get(&group_sizes[i]).unwrap().len() >= group_sizes[i] as usize {
                groups.push(group_map.get(&group_sizes[i]).unwrap().clone());
                group_map.get_mut(&group_sizes[i]).unwrap().clear();
            }
        }
        groups
    }
}
