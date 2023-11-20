/*
Question:
You are given a 0-indexed array of strings garbage where garbage[i] represents the assortment of garbage at the ith house. 
garbage[i] consists only of the characters 'M', 'P' and 'G' representing one unit of metal, paper and glass garbage respectively. 
Picking up one unit of any type of garbage takes 1 minute.

You are also given a 0-indexed integer array travel where travel[i] is the number of minutes needed to go from house i to house i + 1.

There are three garbage trucks in the city, each responsible for picking up one type of garbage. 
Each garbage truck starts at house 0 and must visit each house in order; however, they do not need to visit every house.

Only one garbage truck may be used at any given moment. 
While one truck is driving or picking up garbage, the other two trucks cannot do anything.

Return the minimum number of minutes needed to pick up all the garbage.
*/

// Approach:
// 1) Make a hashmap to store the last occurance indicies of each garbage type
// 2) Make a variable to store the time and initialize it with the sum of all strings in the
//    garbage vector
// 3) Loop over the garbage vector in reverse until the gb map is filled or the vector is finished
//      Each iteration, we go over all characters of the string and if they are not in the app then
//      we add it with the current index as the value
// 4) Transform travel into a prefix sum array using map() and a sum accumulator
// 5) Go over the garbage types in gb and for each type we add the prefix sum of travel upto that
//    index to time
// 6) Return the final time

use std::collections::HashMap;

impl Solution {
    pub fn garbage_collection(garbage: Vec<String>, travel: Vec<i32>) -> i32 {
        let mut gb = HashMap::new();
        let mut time = garbage.iter().fold(0,|acc,x| acc + x.len()) as i32;
        let mut i = garbage.len() as i32 - 1;
        while gb.len() != 3 && i >= 0 {
            for ch in garbage[i as usize].chars() {
                if gb.get(&ch).is_none() {
                    gb.insert(ch,i as usize);
                }
            }
            i -= 1;
        }
        let mut sum = 0;
        let travel: Vec<i32> = travel.iter().map(|x| {
            sum += x;
            sum
        }).collect();
        for i in gb.values() {
            time += *travel.get(i-1).unwrap_or(&0);
        }
        time
    }
}
