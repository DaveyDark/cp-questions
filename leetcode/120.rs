/*
Question:
Given a triangle array, return the minimum path sum from top to bottom.
For each step, you may move to an adjacent number of the row below. 
More formally, if you are on index i on the current row, 
you may move to either index i or index i + 1 on the next row.
*/

// Approach:
// We calculate the sum of the path travelled so far for each element in each row and assign it that value 
// and at the bottom row the minimum value will represent the minimum path sum
// 1) We make triange mutable and the loop through the rows, starting from the second one
// 2) for each row we go through the elements. 
//    If it's an edge element then we just add the value of the corresponding edge element in the upper row to it.
// 3) Otherwise we calulate the sum of that element with the jth and j-1th elements and assign it the smaller of the two sums.
// 4) At the end we return the minimum element in the last row

impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let mut triangle = triangle.clone();
        for i in 1..triangle.len() {
            for j in 0..triangle[i as usize].len() {
                if j == triangle[i as usize].len()-1 || j == 0 {
                    triangle[i][j] += *triangle[i as usize - 1].get(j).unwrap_or(&0) + *triangle[i as usize - 1].get(j-1).unwrap_or(&0);
                    continue;
                }
                let sum1 = *triangle[i as usize - 1].get(j-1).unwrap() + triangle[i][j];
                let sum2 = *triangle[i as usize - 1].get(j).unwrap() + triangle[i][j];
                triangle[i][j] = if sum1 > sum2 {sum2} else {sum1};
            }
        }
        *triangle[triangle.len() - 1].iter().min().unwrap_or(&0)
    }
}
