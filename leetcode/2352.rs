/*
Question:
Given a 0-indexed n x n integer matrix grid, return the number of pairs (ri, cj) such that row ri and column cj are equal.
A row and column pair is considered equal if they contain the same elements in the same order (i.e., an equal array).
*/

// Approach:
// 1) We make a transpose of the grid and store it
// 2) We make a hashmap to store frequency of each row
// 3) Then we go over the grid and hash each row and calculate it's frequency
// 4) Then we go over the transposed grid and if a hashed row matches one in the hashmap,
//    we add it's frequency to the count
// 5) We return the count at the end

use std::collections::HashMap;

impl Solution {
    fn transpose(grid: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut grid = grid.clone();
        for i in 0..grid.len() {
            for j in 0..i {
                let mut tmp = grid[i][j];
                grid[i][j] = grid[j][i];
                grid[j][i] = tmp;
            }
        }
        grid
    }
    pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
        let mut transpose = Self::transpose(&grid);
        let mut map: HashMap<&Vec<i32>,i32> = HashMap::new();
        let mut cnt = 0;

        for row in grid.iter() {
            *map.entry(row).or_insert(0) += 1;
        }

        for row in transpose.iter() {
            cnt += *map.entry(row).or_insert(0);
        }

        cnt
    }
}
