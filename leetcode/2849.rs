/*
Question:
You are given four integers sx, sy, fx, fy, and a non-negative integer t.

In an infinite 2D grid, you start at the cell (sx, sy). Each second, you must move to any of its adjacent cells.

Return true if you can reach cell (fx, fy) after exactly t seconds, or false otherwise.

A cell's adjacent cells are the 8 cells around it that share at least one corner with it. You can visit the same cell several times.
*/

// Approach:
// 1) If the destination is the same as the start cell and t is 1 then we return false
// 2) Otherwise we return true if the times is more than the max distance between the cells

impl Solution {
    pub fn is_reachable_at_time(sx: i32, sy: i32, fx: i32, fy: i32, t: i32) -> bool {
        if sx == fx && sy == fy && t == 1 {
            false
        } else {
            t >= (sx-fx).abs().max((sy-fy).abs())
        }
    }
}
