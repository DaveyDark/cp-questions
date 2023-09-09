/*
Question:
Given an array of intervals intervals where intervals[i] = [starti, endi], 
return the minimum number of intervals you need to remove to make the rest of the intervals non-overlapping.
*/

// Approach:
// 1) Sort the intervals by the end point
// 2) Make varibles to store count of intervals to remove and the end of the last interval
// 3) Loop over given intervals
//     If the start of current interval overlaps the end of the last one, we have to remove it so
//     we increment the counter
//     Otherwise we set the end to the end of the current interval
// 4) Return the counter

impl Solution {
pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
    intervals.sort_unstable_by_key(|int| int[1]);
    let mut cnt = 0;
    let mut end = i32::MIN;
    for pair in intervals {
        if pair[0] < end {
            cnt += 1;
            continue;
        }
        end = pair[1];
    }
    cnt
}
}
