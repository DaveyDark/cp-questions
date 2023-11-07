/*
Question:
You are playing a video game where you are defending your city from a group of n monsters. 
You are given a 0-indexed integer array dist of size n, where dist[i] is the initial distance 
in kilometers of the ith monster from the city.

The monsters walk toward the city at a constant speed. 
The speed of each monster is given to you in an integer array speed of size n, 
where speed[i] is the speed of the ith monster in kilometers per minute.

You have a weapon that, once fully charged, can eliminate a single monster. 
However, the weapon takes one minute to charge. The weapon is fully charged at the very start.

You lose when any monster reaches your city. 
If a monster reaches the city at the exact moment the weapon is fully charged, 
it counts as a loss, and the game ends before you can use your weapon.

Return the maximum number of monsters that you can eliminate before you lose, 
or n if you can eliminate all the monsters before they reach the city.
*/

// Approach:
// 1) Make a vector to store the time needed to each monster to reach the city
// 2) Loop through given monsters and calculate the time for each one and push it into times
// 3) Sort the times vector
// 4) Loop over the sorted times until times[i] <= i, keeping track of the monsters killed and
//    incrementing it every iteration
// 5) Return the kills

impl Solution {
    pub fn eliminate_maximum(dist: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut times = vec!();
        for i in 0..dist.len() {
            times.push((dist[i] as f64 / speed[i] as f64).ceil() as i32);
        }
        times.sort_unstable();
        let mut kills = 0;
        for i in 0..times.len() {
            if times[i] <= i as i32 {break}
            kills += 1;
        }
        kills
    }
}
