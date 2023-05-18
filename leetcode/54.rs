/*
Question:
Given an m x n matrix, return all elements of the matrix in spiral order.
*/

// Approach:
// 1) We use 4 variables to keep track of the boundries the loops should iterate in, and break the main loop if they cross each other
// 2) In the main loop, we go thorugh 4 loops to traverse the matrix in a spiral pattern:
//    - from left to right
//    - from top+1 to bottom
//    - from right-1 to left
//    - from bottom-1 to top
// 3) We also check if the top and bottom or left and right are coinciding after the 2nd loop and break if they are
//    This is to resolve edge cases of the remaining region or the whole matrix being only 1 row or only 1 column

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut soln = Vec::new();
        let mut left = 0;
        let mut top = 0;
        let mut right = matrix[0].len() as i32 - 1;
        let mut bottom = matrix.len() as i32 -1;
        while top <= bottom && left <= right {
            //go right
            for j in left..=right {
                soln.push(matrix[top as usize][j as usize]);
            }
            //go down
            for i in top+1..=bottom {
                soln.push(matrix[i as usize][right as usize]);
            }
            if(top>=bottom || left>=right){break;}
            //go left
            for j in (left..right).rev() {
                soln.push(matrix[bottom as usize][j as usize]);
            }
            top+=1;
            //go up
            for i in (top..bottom).rev() {
                soln.push(matrix[i as usize][left as usize]);
            }
            left+=1;
            bottom-=1;
            right-=1;
        }
        soln
    }
}
