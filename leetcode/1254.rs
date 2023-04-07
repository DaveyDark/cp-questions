/*
Question:
Given a 2D grid consists of 0s (land) and 1s (water).  
An island is a maximal 4-directionally connected group of 0s 
and a closed island is an island totally (all left, top, right, bottom) surrounded by 1s.

Return the number of closed islands.
*/

// Approach:
// 1) Loop through all middle elements of the grid
// 2) If there is land at any element, then we call the terraform function on it
//    Terraform function recursively checks all connected land tiles and returns true if it's an island
//    Terraform also changes the 0s to 2s so it doesn't get into an infinite loop and each island
//    Only gets terraformed once
// 3) If the terraform returns true, we increase the counter

fn terraform(grid: &mut Vec<Vec<i32>>, y: usize, x: usize) -> bool{
    grid[y][x] = 2;
    let mut ret = true;
    for i in -1..=1i32{
        for j in -1..=1i32{
            if !((i == 0)^(j == 0)) {
                continue;
            }
            if (y as i32 + i) < 0 || (x as i32 + j) < 0 || (y + i as usize) >= grid.len() || (x + j as usize) >= grid[0].len() {
                //edge touching element
                ret = false;
                continue;
            }
            if grid[y+i as usize][x+j as usize] == 0 {
                //adjacent land
                if !terraform(grid,y+i as usize,x+j as usize){
                    //if it's touching edge
                    ret = false;
                }
            }
        }
    }
    ret
}
impl Solution {
    pub fn closed_island(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid.clone();
        let mut islands = 0;
        for i in 1..grid.len()-1 {
            for j in 1..grid[0].len()-1 {
                if grid[i][j] == 0 {
                    if terraform(&mut grid,i,j) {
                        islands+=1;
                    }
                }
            }
        }
        islands
    }
}
