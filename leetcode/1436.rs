/*
Question:
You are given the array paths, where paths[i] = [cityAi, cityBi] means there exists a
direct path going from cityAi to cityBi. Return the destination city, that is,
the city without any path outgoing to another city.

It is guaranteed that the graph of paths forms a line without any loop, therefore,
there will be exactly one destination city.
*/

// Approach:
// 1) Create a hashmap to store cities and their frequency
// 2) Iterate over paths and increment the frequency of the first city and insert the second city
//    if it isn't in the map
// 3) Iterate over the map and return the city with frequency 0

use std::collections::HashMap;

impl Solution {
    pub fn dest_city(paths: Vec<Vec<String>>) -> String {
        let mut cities = HashMap::new();
        for path in &paths {
            *cities.entry(&path[0]).or_insert(0) += 1;
            if cities.get(&path[1]).is_none() {
                cities.insert(&path[1], 0);
            }
        }
        cities
            .iter()
            .filter(|(ct, fr)| **fr == 0)
            .next()
            .unwrap()
            .0
            .to_string()
    }
}
