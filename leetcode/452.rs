/*
Question:
here are some spherical balloons taped onto a flat wall that represents the XY-plane. 
The balloons are represented as a 2D integer array points where 
points[i] = [xstart, xend] denotes a balloon whose horizontal diameter stretches between xstart and xend. 
You do not know the exact y-coordinates of the balloons.

Arrows can be shot up directly vertically (in the positive y-direction) from different points along the x-axis. 
A balloon with xstart and xend is burst by an arrow shot at x if xstart <= x <= xend. 
There is no limit to the number of arrows that can be shot. A shot arrow keeps traveling up infinitely, bursting any balloons in its path.

Given the array points, return the minimum number of arrows that must be shot to burst all balloons.
*/

// Approach:
// 1) Sort the given intervals by the end
// 2) Make variables to track the end of the last interval and the number of arrows needed, which
//    will be 1 to start
// 3) Loop through given points
//     If the last end point is outside the current interval we reset end and increment arrows
// 4) Return the number of arrows

impl Solution {
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort_unstable_by_key(|point| point[1]);
        let mut end = points[0][1];
        let mut arrows = 1;
        for point in points {
            if(end < point[0]) {
                arrows += 1;
                end = point[1];
            }
        }
        arrows
    }
}
