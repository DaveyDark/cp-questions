/*
Question:
We have a wooden plank of the length n units. Some ants are walking on the plank, 
each ant moves with a speed of 1 unit per second. Some of the ants move to the left, the other move to the right.

When two ants moving in two different directions meet at some point, 
they change their directions and continue moving again. Assume changing directions does not take any additional time.

When an ant reaches one end of the plank at a time t, it falls out of the plank immediately.

Given an integer n and two integer arrays left and right, 
the positions of the ants moving to the left and the right, 
return the moment when the last ant(s) fall out of the plank.
*/

// Approach:
// The ants colliding makes no difference so it can be ignored. So the time can be directly
// calculated by calculating the max distance travelled by any of the ants
// 1) Make a variable to store the max distance
// 2) Traverse the left array and set max_dist to the max of each element
// 3) Traverse the right array and set the max_dist to the max of n - each element

impl Solution {
    pub fn get_last_moment(n: i32, left: Vec<i32>, right: Vec<i32>) -> i32 {
        let mut max_dist = 0;
        for ant in left {
            max_dist = max_dist.max(ant);
        }
        for ant in right {
            max_dist = max_dist.max(n-ant);
        }
        max_dist
    }
}
