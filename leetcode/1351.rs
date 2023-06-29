/*
Question:
Given a m x n matrix grid which is sorted in non-increasing order both row-wise and column-wise, return the number of negative numbers in grid.
*/

// Approach:
// 1) We make a counter to count the negative numbers and variables left,right and mid for the binary searching
//    Left and right are zero while mid is initialized to the length of a row, because right has to be set to mid-1 later
// 2) Then we go over each of the rows in the grid and for each row,
//    left is reset to 0, and right is set to mid which holds the index of the last positive element of the last row
//    We perform a binary search to look for the last positive element in the row, then do some correction after the loop to make sure the index is corrent
//    Then we subtract the index of the last positive element from the length and add the value to the counter
// 3) At the end we reutrn the counter

impl Solution {
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        let mut ctr = 0;
        let (mut left, mut right): (i32,i32) = (0,0);
        let mut mid: i32 = grid[0].len() as i32;
        for row in grid.iter() {
            left = 0;
            right = mid-1;
            while left <= right {
                mid = (left+right)/2;
                if row[mid as usize] >= 0 {
                    left = (mid+1);
                } else {
                    right = (mid-1);
                }
            }
            if row[mid as usize] >= 0 {mid += 1}
            ctr += row.len() as i32 - mid;
        }
        ctr
    }
}
