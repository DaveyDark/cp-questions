/*
Question:
You are given an m x n matrix mat and two integers r and c representing the number of rows and the number of columns of the wanted reshaped matrix.
The reshaped matrix should be filled with all the elements of the original matrix in the same row-traversing order as they were.
If the reshape operation with given parameters is possible and legal, output the new reshaped matrix; Otherwise, output the original matrix.
*/

// Approach:
// 1) The original matrix is returned if the reshaping is not possible
// 2) We loop through all the elements in the matrix
// 3) If the counter is >= given size then we push the row into the new matrix and reset the row vector
// 3) We push elements into the row vector and increment the counter
// 4) Outside the loop, We push the last row into the new matrix and return it

impl Solution {
    pub fn matrix_reshape(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
        if (c*r) != (mat.len() * mat[0].len()) as i32 {
            return mat;
        }
        let mut cnt = 0;
        let mut row: Vec<i32> = Vec::new();
        let mut new_mat: Vec<Vec<i32>> = Vec::new();
        for vector in &mat {
            for item in vector {
                if cnt >= c {
                    cnt = 0;
                    new_mat.push(row);
                    row = Vec::new();
                }
                cnt += 1;
                row.push(*item);
            }
        }
        new_mat.push(row);
        new_mat
    }
}
