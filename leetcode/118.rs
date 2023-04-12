/*
Question:
Given an integer numRows, return the first numRows of Pascal's triangle.
In Pascal's triangle, each number is the sum of the two numbers directly above it
*/

// Approach:
// Edge elements of any row of pascal's triangle are always 1
// Any middle element of index j of row i will be the sum of index j-1 and j of row i-1
// 1) We loop through the given number of rows
// 2) For each row we insert the edge elements as 1
// 3) The middle elements are calculated as explained above and pushed
// 4) The row is pushed into the triangle at the end of the inner loop
// 5) The final triangle matrix is returned

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut triangle: Vec<Vec<i32>> = Vec::new();
        let mut row: Vec<i32> = Vec::new();
        for i in 0..num_rows {
            for j in 0..=i {
                if j == 0 || j == i {
                    row.push(1);
                    continue;
                }
                row.push(triangle[i as usize - 1][j as usize - 1] + triangle[i as usize - 1][j as usize]);
            }
            triangle.push(row);
            row = Vec::new();
        }
        triangle
    }
}
