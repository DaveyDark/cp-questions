/*
Question:
You are given an array of n pairs pairs where pairs[i] = [lefti, righti] and lefti < righti.

A pair p2 = [c, d] follows a pair p1 = [a, b] if b < c. A chain of pairs can be formed in this fashion.

Return the length longest chain which can be formed.

You do not need to use up all the given intervals. You can select pairs in any order.
*/

// Approach:
// 1) Sort the given pairs by the end point
// 2) Then make a recursive function chain() to calculate the longest chain
//      In chain(), we first check if the current index is more than length of pairs and return 0
//      is it is, this is the base case
//      Otherwise we check if the start of the current pair is more than the end of the last pair,
//      if it is then we add 1 and recursively call chain() with the next index and updated end
//      Otherwise we just use chain() again with the next index but the same end
// 3) Call chain() and return the value returned by it

impl Solution {
    fn chain(pairs: &Vec<Vec<i32>>, i: usize, end: i32) -> i32 {
        if i == pairs.len() {return 0;}
        if pairs[i][0] > end {
            return 1 + Self::chain(pairs, i+1, pairs[i][1]);
        }
        return Self::chain(pairs, i+1, end);
    } 
    pub fn find_longest_chain(mut pairs: Vec<Vec<i32>>) -> i32 {
        pairs.sort_unstable_by_key(|pair| pair[1]);
        Self::chain(&pairs, 0, i32::MIN)
    }
}
