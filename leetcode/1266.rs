/*
Question:
On a 2D plane, there are n points with integer coordinates points[i] = [xi, yi]. 
Return the minimum time in seconds to visit all the points in the order given by points.

You can move according to these rules:
    In 1 second, you can either:
        move vertically by one unit,
        move horizontally by one unit, or
        move diagonally sqrt(2) units (in other words, move one unit vertically then one unit horizontally in 1 second).
    You have to visit the points in the same order as they appear in the array.
    You are allowed to pass through points that appear later in the order, but these do not count as visits.
*/

// Approach:
// 1) Make windows of size 2 over the points since we need to loop over each(overlapping) pair of
//    points
// 2) Then map each window to the chebyshev distance between the two points i.e. the max of the
//    absolute distances on beoth axes.
// 3) Then sum the iterator and return it.

impl Solution {
    pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
        points
        .windows(2)
        .map(
            |win| (win[0][0] - win[1][0]).abs().max((win[0][1] - win[1][1]).abs())
        )
        .sum()
    }
}
