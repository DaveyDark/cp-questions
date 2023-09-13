/*
Question:
There are n children standing in a line. Each child is assigned a rating value given in the integer array ratings.

You are giving candies to these children subjected to the following requirements:
    Each child must have at least one candy.
    Children with a higher rating get more candies than their neighbors.

Return the minimum number of candies you need to have to distribute the candies to the children.
*/

// Approach:
// 1) Make a vector to store candies for each child and fill it with 1s
// 2) Go through the given vector from left to right and if the rating of current child is more
//    than the last one we set their candies to the last child's candies + 1
// 3) Go through the given vector from right to left and if the rating of current child is more
//    than the last one and they have less candies than them then we set their candies to the last
//    child's + 1
// 4) Return the sum of the candies vector

impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let mut candies = vec!(1; ratings.len());
        for i in 0..ratings.len() {
            if i > 0 && ratings[i] > ratings[i-1] {
                candies[i] = candies[i-1] + 1;
            }
        }
        for i in (0..ratings.len()).rev() {
            if i < ratings.len()-1 && ratings[i] > ratings[i+1]
            && candies[i] <= candies[i+1] {
                candies[i] = candies[i+1] + 1;
            }
        }
        candies.iter().sum()
    }
}
