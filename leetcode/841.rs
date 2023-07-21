/*
Question:
There are n rooms labeled from 0 to n - 1 and all the rooms are locked except for room 0. 
Your goal is to visit all the rooms. However, you cannot enter a locked room without having its key.

When you visit a room, you may find a set of distinct keys in it. Each key has a number on it, 
denoting which room it unlocks, and you can take all of them with you to unlock the other rooms.

Given an array rooms where rooms[i] is the set of keys that you can obtain if you visited room i, 
return true if you can visit all the rooms, or false otherwise.
*/

// Approach:
// keys will be a vector that holds the keys we have for different rooms
// unvisited will be a hashset representing the unvisited rooms
// We use a function add_keys() that goes through the keys avvaiable in a room and 
// adds them to keys if are for an unvisited room and we don't already have a key for that room
// 1) Init keys to an empty vector and unvisited to a range from 1 to rooms.len()
// 2) Add the initial keys that we get from the first room to keys
// 3) Loop until we run out of keys
//    Every iteration, we take a key out from keys and add the keys from that room to keys using add_keys()
//    We also remove the room from the unvisited set
// 4) At the end when all keys are spent, if there are still any unvisited rooms left, return false, otherwise return true

use std::collections::HashSet;

impl Solution {
    fn add_keys(keys: &mut Vec<usize>, unvisited: &HashSet<usize>, room: &Vec<i32>) {
        for &key in room.iter() {
            if !unvisited.contains(&(key as usize)) || keys.contains(&(key as usize)) {continue}
            keys.push(key as usize);
        }
    }

    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        let mut keys: Vec<usize> = vec!();
        let mut unvisited: HashSet<usize> = (1..rooms.len()).collect();
        Self::add_keys(&mut keys,&unvisited,&rooms[0]);
        while keys.len() > 0 {
            let room = keys.pop().unwrap();
            unvisited.remove(&(room as usize));
            Self::add_keys(&mut keys, &unvisited, &rooms[room]);
        }
        unvisited.is_empty()
    }
}
