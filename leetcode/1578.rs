/*
Question:
Alice has n balloons arranged on a rope.
You are given a 0-indexed string colors where colors[i] is the color of the ith balloon.

Alice wants the rope to be colorful.
She does not want two consecutive balloons to be of the same color, so she asks Bob for help.
Bob can remove some balloons from the rope to make it colorful.
You are given a 0-indexed integer array neededTime where neededTime[i] is the time (in seconds)
that Bob needs to remove the ith balloon from the rope.

Return the minimum time Bob needs to make the rope colorful.
*/

// Approach:
// 1) Maintain a sum of time and max time for each color, also keep track of last color and time so
//    far.
// 2) Iterate over the colors and time, if current color is same as last color, update sum and max.
//   Else, add the difference of sum and max to time and update sum and max to be the current time.
// 3) After the iteration, add the difference of sum and max to time.
// 4) Return time.

impl Solution {
    pub fn min_cost(colors: String, mut needed_time: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut max = 0;
        let mut last = '-';
        let mut time = 0;

        for (ch, t) in colors.chars().zip(needed_time.into_iter()) {
            if ch == last {
                sum += t;
                max = max.max(t);
            } else {
                time += (sum - max);
                sum = t;
                max = t;
            }
            last = ch;
        }
        time += (sum - max);

        time
    }
}
