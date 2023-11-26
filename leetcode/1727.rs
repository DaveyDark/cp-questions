/*
Question:
You are given a binary matrix matrix of size m x n, and you are allowed to rearrange the columns of the matrix in any order.

Return the area of the largest submatrix within matrix where every element of the submatrix is 1 after reordering the columns optimally.
*/

// Appraoch:
// 1) Make a streaks vector to track the streak of consecutive 1s per row, init to all 0s
// 2) Loop over each element in the matrix and update streaks, inrementing it if it is a 1 or
//    resetting it to 0 if it's a 0, then setting the value of the element to the corresponding
//    streak
// 3) Make a variable to track the max area so far then loop over each row and clone and sort it
//      Then loop over the sorted row and calculate the area using row[i] * (i+1) and set max_area if needed
// 4) Return the max area

impl Solution {
    pub fn largest_submatrix(mut matrix: Vec<Vec<i32>>) -> i32 {
        let mut streaks = vec!(0; matrix[0].len());
        for i in 0..matrix.len() {
            for j in 0..matrix[i].len() {
                if matrix[i][j] == 1 {
                    streaks[j] += 1;
                } else {
                    streaks[j] = 0;
                }
                matrix[i][j] = streaks[j];
            }
        }
        let mut max_area = 0;
        for row in &matrix {
            let mut r = row.clone();
            r.sort_unstable_by_key(|x| -x);
            for i in 0..r.len() {
                max_area = max_area.max(r[i] * (i as i32 + 1));
            }
        }
        max_area
    }
}
