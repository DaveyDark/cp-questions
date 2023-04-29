/*
Question:
You are given an m x n binary matrix grid. An island is a group of 1's (representing land) connected 4-directionally (horizontal or vertical.) 
You may assume all four edges of the grid are surrounded by water.
The area of an island is the number of cells with a value 1 in the island.
Return the maximum area of an island in grid. If there is no island, return 0.
*/

// Approach:
// 1) We track the max area in a var called max
// 2) Then we go over the grid and if we encounter any land, we call the calc_area function on it
//    The calc_area function calculates are by recursively calling itself on adjacent land tiles
//    It also converts the checked tiles to 2 to avoid infinite looping
// 3) We compare the area of each island to max and set max accordingly and return it at the end

impl Solution {
    pub fn calc_area(grid: &mut Vec<Vec<i32>>, y: i32, x: i32, sum: &mut i32) {
        grid[y as usize][x as usize] = 2;
        *sum += 1;
        for i in -1..2 {
            for j in -1..2 {
                if i != 0 && j != 0 {continue;}
                if y+i < 0 || y+i >= grid.len() as i32 || x+j < 0 || x+j >= grid[0].len() as i32 {continue;}
                if grid[(y+i) as usize][(x+j) as usize] == 1 {Solution::calc_area(grid,y+i,x+j,sum);}
            }
        }
    }
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid.clone();
        let mut max = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    let mut area: i32 = 0;
                    Solution::calc_area(&mut grid,i as i32,j as i32,&mut area);
                    if(area > max){max = area;}
                }
            }
        }
        max
    }
}
