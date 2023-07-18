/*
Question:
Design a data structure that follows the constraints of a Least Recently Used (LRU) cache.

Implement the LRUCache class:
    LRUCache(int capacity) Initialize the LRU cache with positive size capacity.
    int get(int key) Return the value of the key if the key exists, otherwise return -1.
    void put(int key, int value) Update the value of the key if the key exists. 
    Otherwise, add the key-value pair to the cache. 
    If the number of keys exceeds the capacity from this operation, evict the least recently used key.

The functions get and put must each run in O(1) average time complexity.
*/

// Approach:
// Ideally we should use a linked list for storing the history/cache of the LRU, but rust doesn't have Linked Lists so we can just use a VecDeque
// Close enough except it's less performant
// And for the actual memory we use a HashMap
// And the LRU has an i32 that represents the max capacity
// 
// For creating a new object we can just make an empty VecDeque and HashMap and set cap to given capacity
// 
// When getting from the LRU, we check if the key is present in the memory
//   If it is then we remove it's entry from the cache and insert it again at the back of the cache
//     And return the respective value of the key from memory
//   Otherwise if the key is not present, we just return -1
//
// When putting value into the LRU, we check if the key is already in the memory, and clear it's entry in the cache if it is
// If it isn't and the memory is full, then we remove the last entry from the cache and the respective value from the memory as well
// And after either case, we push the key to the back of the cache and insert the given key/value into memory

use std::collections::{ HashMap, VecDeque };

struct LRUCache {
    cache: VecDeque<i32>,
    mem: HashMap<i32,i32>,
    cap: i32,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        LRUCache {
            cache: VecDeque::new(),
            mem: HashMap::new(),
            cap: capacity,
        }
    }
    
    fn get(&mut self, key: i32) -> i32 {
        if self.mem.contains_key(&key) {
            self.cache.remove(self.cache.iter().position(|&x| x == key).unwrap());
            self.cache.push_back(key);
            *self.mem.get(&key).unwrap()
        } else { -1 }
    }
    
    fn put(&mut self, key: i32, value: i32) {
        if self.mem.contains_key(&key) {
            self.cache.remove(self.cache.iter().position(|&x| x == key).unwrap());
        } else if self.cache.len() == self.cap as usize {
            let x = self.cache.pop_front().unwrap();
            self.mem.remove(&x);
        }
        self.cache.push_back(key);
        self.mem.insert(key,value);
    }
}
