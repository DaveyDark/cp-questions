/*
Question:
Given n points on a 2D plane where points[i] = [xi, yi],
Return the widest vertical area between two points such that no points are inside the area.

A vertical area is an area of fixed-width extending infinitely along the y-axis (i.e., infinite height).
The widest vertical area is the one with the maximum width.

Note that points on the edge of a vertical area are not considered included in the area.
*/

// Approach:
// 1) Transform the input into a vector of x values
// 2) Sort the vector
// 3) Loop over the vector and find the max difference between two consecutive values and return it

impl Solution {
    pub fn max_width_of_vertical_area(points: Vec<Vec<i32>>) -> i32 {
        let mut points: Vec<i32> = points.iter().map(|p| p[0]).collect();
        points.sort_unstable();
        points.windows(2).fold(0, |max, w| max.max(w[1] - w[0]))
    }
}
