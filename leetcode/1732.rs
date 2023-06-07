/*
Question:
There is a biker going on a road trip. The road trip consists of n + 1 points at different altitudes. 
The biker starts his trip on point 0 with altitude equal 0.
You are given an integer array gain of length n where gain[i] is the net gain in altitude between points i
and i + 1 for all (0 <= i < n). Return the highest altitude of a point.
*/

// Approach:
// 1) We keep a variable to track the altitude and one to track max altitude
// 2) We go through the gain vector and update altitude by the current element
//    Then we set max to the greater of max and altitude
// 3) At the end we return the max altitude

impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        let mut altitude = 0;
        let mut max = 0;
        for alt in gain.iter() {
            altitude += *alt;
            max = max.max(altitude);
        }
        max
    }
}
