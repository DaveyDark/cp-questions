/*
Question:
There are n kids with candies. 
You are given an integer array candies, where each candies[i] represents the number of candies the ith kid has, 
and an integer extraCandies, denoting the number of extra candies that you have.
Return a boolean array result of length n, where result[i] is true if, after giving the ith kid all the extraCandies, 
they will have the greatest number of candies among all the kids, or false otherwise.
Note that multiple kids can have the greatest number of candies.
*/

// Approach:
// 1) We get the max number of candies from the given vector and store it
// 2) Then we loop through the vector and check if the current element is >= max after adding the extra candies
//    and push true to sol vector if it does, otherwise push false
// 3) We return the sol vector at the end

impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let max = *candies.iter().max().unwrap();
        let mut sol = Vec::new();

        for n in candies.iter(){
            sol.push(if n+extra_candies >= max {true} else {false})
        }

        sol
    }
}
