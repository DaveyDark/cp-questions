/*
Question:
Given an integer rowIndex, return the rowIndexth (0-indexed) row of the Pascal's triangle.

In Pascal's triangle, each number is the sum of the two numbers directly above it as shown:
*/

// Approach:
// 1) Make two vectors to store the current and last rows, and set the last row to [1]
// 2) If the requested index is 0 return a vector with just 1
// 3) Loop from 0 to row_index
//    Add 1 to the front of current row
//    For the middle elements, curr[i]  will be the sum of last[i-1] and last[i]
//    Then add another end at the end of the row
//    Then we make the last row equal to current row and reset the curr row
// 4) After the loop we return the last row

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut last = vec!(1);
        let mut curr = Vec::new();
        if row_index == 0 {
            return vec!(1);
        }
        for i in 0..=row_index {
            curr.push(1);
            for j in 1..i {
                curr.push(last[j as usize] + last[j as usize -1]);
            }
            curr.push(1);
            last = curr;
            curr = Vec::new();
        }
        last
    }
}
