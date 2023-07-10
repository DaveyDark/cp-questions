/*
Question:
You are given an n x n 2D matrix representing an image, rotate the image by 90 degrees (clockwise).
You have to rotate the image in-place, which means you have to modify the input 2D matrix directly. 
DO NOT allocate another 2D matrix and do the rotation.
*/

// Approach 1:
// We first transpose the matrix and then reverse it's rows
// 1) First we transpose the matrix by looping through the lower triangular half of the matrix and swapping the (i,j) element with the (j,i) element
// 2) Then we again loop through the matrix rows and reverse each row by swapping every element of the row with the opposite element

class Solution {
    public void rotate(int[][] matrix) {
        for(int i=0; i<matrix.length; i++) {
            for(int j=0; j<i; j++) {
                int tmp = matrix[i][j];
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = tmp;
            }
        }
        for(int i=0; i<matrix.length; i++) {
            for(int j=0; j<matrix.length/2; j++) {
                int tmp = matrix[i][j];
                matrix[i][j] = matrix[i][matrix.length-1-j];
                matrix[i][matrix.length-1-j] = tmp;
            }
        }
    }
}

// Approach 2:
// Instead of using nested for loops twice, we can do the rotation in one nested for loop if we do both the transpose and reverse calculation in the same loop
// For explaination of the calculation, consider a 4x4 matrix and suppose we want to calculate the final position of these given indices
// 0,1 -> 1,3
// 1,3 -> 3,2
// 3,2 -> 2,0
// 2,0 -> 0,1
// We can see a pattern here, the previous j becomes the new i and the previous i is inverted and made the new j
// 1) We go through the upper triangle of the first quadrant of the matrix, we only loop over this part to avoid rotating any cell more than once
//    Then for each cell, we store it's value in a temp variable, then calculate the new index, and swap the value of the temp variable and that index
//    We do this 4 times to cover all 4 of the quadrants of the matrix
class Solution {
    public void rotate(int[][] matrix) {
        for(int i=0; i<matrix.length/2; i++){
            for(int j=i; j<matrix.length-1-i; j++){
                int t = matrix[i][j];
                for(int k=0; k<4; k++) {
                    int tmp_index = j;
                    j = (matrix.length-1-i);
                    i = tmp_index;
                    int tmp = t;
                    t = matrix[i][j];
                    matrix[i][j] = tmp;
                }
            }
        }
    }
}
