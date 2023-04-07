/*
Question:
You are given an m x n binary matrix grid, where 0 represents a sea cell and 1 represents a land cell.

A move consists of walking from one land cell to another adjacent (4-directionally) land cell or walking off the boundary of the grid.

Return the number of land cells in grid for which we cannot walk off the boundary of the grid in any number of moves.
*/

// Approach:
// 1) Loop through all middle elements of the grid
// 2) If there is land at any element, then we call the terraform function on it
//    Terraform function recursively checks all connected land tiles and returns counts the land tiles
//    Terraform changes the counter to -1 if the lang chunk is connected to the edge
//    Terraform also changes the 0s to 2s so it doesn't get into an infinite loop and each island
//    Only gets terraformed once
// 3) If the terraform returns > 0, we increase the counter by the returned value

fn terraform(grid: &mut Vec<Vec<i32>>, y: usize, x: usize, cnt: &mut i32) {
    grid[y][x] = 2;
    for i in -1..=1i32{
        for j in -1..=1i32{
            if !((i == 0)^(j == 0)) {
                continue;
            }
            if (y as i32 + i) < 0 || (x as i32 + j) < 0 || (y + i as usize) >= grid.len() || (x + j as usize) >= grid[0].len() {
                //edge touching element
                *cnt = -1;
                continue;
            }
            if grid[y+i as usize][x+j as usize] == 1 {
                //adjacent land
                if *cnt >= 0 {
                    *cnt += 1;
                    println!("land at {} {}",y+i as usize,x+j as usize);
                }
                terraform(grid,y+i as usize,x+j as usize,cnt);
            }
        }
    }
}
impl Solution {
    pub fn num_enclaves(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid.clone();
        let mut enclaves = 0;
        for i in 1..grid.len()-1 {
            for j in 1..grid[0].len()-1 {
                if grid[i][j] == 1 {
                    let mut cnt: i32 = 1;
                    terraform(&mut grid,i,j,&mut cnt);
                    if cnt > 0 {
                        enclaves += cnt;
                    }
                }
            }
        }
        enclaves
    }
}
