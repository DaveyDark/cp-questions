/*
Question:
You are given an array people where people[i] is the weight of the ith person, 
and an infinite number of boats where each boat can carry a maximum weight of limit. 
Each boat carries at most two people at the same time, 
provided the sum of the weight of those people is at most limit.
Return the minimum number of boats to carry every given person.
*/


// Approach:
// 1) Sort given vector in ascending order of weight
// 2) Get left and right of the vector and loop until left <= right
// 3) Check if the left + right person is <= weight limit and send them if it is
// 4) If it isn't then send the heavier(right) person

impl Solution {
    pub fn num_rescue_boats(people: Vec<i32>, limit: i32) -> i32 {
        //sort people vector
        let mut boats = 0;
        let mut people: Vec<i32> = people.clone(); //clone and shadow people to make it mutable
        people.sort(); //sort vector
        let mut left: i32 = 0;
        let mut right: i32 = (people.len()-1) as i32;
        while left <= right {
            boats += 1;
            // check if we can send a pair
            if people[left as usize] + people[right as usize] <= limit {
               left += 1;
               right -= 1;
               continue;
            }
            //otherwise send the heavier person
            right -= 1;
        }
        boats
    }
}
