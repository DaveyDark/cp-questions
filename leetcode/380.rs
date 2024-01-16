/*
Question:
Implement the RandomizedSet class:
    RandomizedSet() Initializes the RandomizedSet object.
    bool insert(int val) Inserts an item val into the set if not present.
      Returns true if the item was not present, false otherwise.
    bool remove(int val) Removes an item val from the set if present.
      Returns true if the item was present, false otherwise.
    int getRandom() Returns a random element from the current set of elements
      (it's guaranteed that at least one element exists when this method is called).
      Each element must have the same probability of being returned.

You must implement the functions of the class such that each function works in average O(1) time complexity.
*/

// Approach:
// The class will have a vector of values "vals" and a hashmap of values to indices "map", along with a random number generator.
// new():
//   return a new instance of the class
// insert():
//   if the value exists in the hashmap, return false
//   otherwise, add the value to the vals and the index of the inserted value to map
// remove():
//   if the value doesn't exist in the map, return false
//   otherwise, we remove the item from the map and then swap_remove the index from vals
//   then if i is still a valid index, we set vals[i] to i in the map
// get_random():
//   generates a random number in the range 0 to vals.len() and returns the number in vals at that index

use rand::Rng;
use std::collections::HashMap;

struct RandomizedSet {
    vals: Vec<i32>,
    map: HashMap<i32, usize>,
    rng: rand::rngs::ThreadRng,
}

impl RandomizedSet {
    fn new() -> Self {
        RandomizedSet {
            vals: Vec::new(),
            map: HashMap::new(),
            rng: rand::thread_rng(),
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        if self.map.contains_key(&val) {
            return false;
        }
        self.map.insert(val, self.vals.len());
        self.vals.push(val);
        true
    }

    fn remove(&mut self, val: i32) -> bool {
        if !self.map.contains_key(&val) {
            return false;
        }
        let i = self.map.remove(&val).unwrap();
        self.vals.swap_remove(i);
        if i < self.vals.len() {
            self.map.insert(self.vals[i], i);
        }
        true
    }

    fn get_random(&mut self) -> i32 {
        let i = self.rng.gen_range(0, self.vals.len());
        self.vals[i]
    }
}
