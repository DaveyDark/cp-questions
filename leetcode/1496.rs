/*
Question:
Given a string path, where path[i] = 'N', 'S', 'E' or 'W',
each representing moving one unit north, south, east, or west, respectively.
You start at the origin (0, 0) on a 2D plane and walk on the path specified by path.

Return true if the path crosses itself at any point, that is,
if at any time you are on a location you have previously visited.
Return false otherwise.
*/

// Approach:
// 1) Make a set of visited points and add the origin to it.
// 2) Iterate over the path and update the current position based on the direction.
// 3) If the current position is already in the set, return true.
// 4) Otherwise, add the current position to the set and continue.
// 5) If the loop completes, return false.

use std::collections::HashSet;

impl Solution {
    pub fn is_path_crossing(path: String) -> bool {
        let mut visited = HashSet::new();
        let mut curr = (0, 0);
        visited.insert(curr.clone());
        for ch in path.chars() {
            match ch {
                'N' => curr.0 -= 1,
                'E' => curr.1 += 1,
                'S' => curr.0 += 1,
                'W' => curr.1 -= 1,
                _ => (),
            }
            if !visited.insert(curr.clone()) {
                return true;
            }
        }
        false
    }
}
