/*
Question:
You are given an m x n integer matrix matrix with the following two properties:
    Each row is sorted in non-decreasing order.
    The first integer of each row is greater than the last integer of the previous row.
Given an integer target, return true if target is in matrix or false otherwise.
You must write a solution in O(log(m * n)) time complexity.
*/

// Approach:
// We basically perform binary search and use two functions to change between 2d and 1d indices
// 1) Init variables for left,right,mid and row length
// 2) We loop until left > right
// 3) Middle element is the average of left and right index
//    It is then converted into a 2d location
// 4) The element at that location is then compared to the target
      If it is greater then left half of the matrix is taken
      If it is lesser then right half of the matrix is taken
      If it is equal then true is returned
// 5) If the element isn't found then false is returned after the loop

fn parse_index(n: i32,row_len: i32) -> (i32,i32) {
    (n/row_len,n%row_len)
}

fn parse_loc(loc: (i32,i32),row_len: i32) -> i32 {
    (loc.0*row_len) + (loc.1)
}

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let mut left: i32 = 0;
        let mut right: i32 = (matrix.len() * matrix[0].len()) as i32 - 1;
        let mut mid: i32 = 0;
        let mut row_len: i32 = matrix[0].len() as i32;
        loop {
            mid = (left+right)/2;
            let mut loc = parse_index(mid,row_len);
            let mut mid_element = matrix[loc.0 as usize][loc.1 as usize];
            if (mid_element == target) {
                return true;
            } else if (mid_element > target) {
                // target left half
                right = parse_loc(loc,row_len) - 1;
            } else {
                // target right half
                left = parse_loc(loc,row_len) + 1;
            }
            if left > right {break;}
        }
        false
    }
}
