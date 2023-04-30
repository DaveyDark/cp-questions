/*
Question:
You are given an m x n grid where each cell can have one of three values:
    0 representing an empty cell,
    1 representing a fresh orange, or
    2 representing a rotten orange.
Every minute, any fresh orange that is 4-directionally adjacent to a rotten orange becomes rotten.
Return the minimum number of minutes that must elapse until no cell has a fresh orange. If this is impossible, return -1.
*/

// Approach:
// 1) We first calculate the amount of fresh oranges in the grid
// 2) Then we loop and each iteration we call the rot function on all rotten oranges
//    The rot function adds fresh oranges that are adjacent to rotten oranges to the rotting vector
// 3) Then we go throught the rotting oranges and turn them into rotten oranges on the grid
// 4) We also subract the amount of oranges that just got rotten from the number of fresh oranges
// 5) If no oranges have rotted this minute then we break out of the loop
// 6) At the end of the interation we increment the number of minutes
// 7) At the end of the function we return mins if no fresh oranges are left, otherwise we return -1

use std::collections::HashSet;

impl Solution {
    pub fn rot(grid: &mut Vec<Vec<i32>>, rotting: &mut HashSet<(i32,i32)>, i: i32, j: i32){
        for y in -1..2 {
            for x in -1..2 {
                if x != 0 && y != 0 {continue;}
                if i+y < 0 || i+y >= grid.len() as i32 || x+j < 0 || x+j >= grid[0].len() as i32 {continue;}
                if grid[(i+y) as usize][(x+j) as usize] == 1 {
                    rotting.insert((i+y,j+x));
                }
            }
        }
    }
    pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid.clone();
        let mut mins = 0;
        let mut fresh = 0;
        let mut rotting: HashSet<(i32,i32)> = HashSet::new();
        for row in &grid {
            for orange in row {
                if *orange == 1 {fresh += 1;}
            }
        }
        loop {
            rotting.clear();
            for i in 0..grid.len() {
                for j in 0..grid[0].len(){
                    if grid[i][j] == 2 {
                        Solution::rot(&mut grid, &mut rotting, i as i32, j as i32);
                    }
                }
            }
            for orange in &rotting {
                grid[orange.0 as usize][orange.1 as usize] = 2;
            }
            fresh -= rotting.len() as i32;
            if rotting.len() == 0 {break; }
            mins += 1;
        }
        if fresh > 0 {-1} else {mins}
    }
}
