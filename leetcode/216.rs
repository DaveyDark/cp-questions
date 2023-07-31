/*
Question:
Find all valid combinations of k numbers that sum up to n such that the following conditions are true:

    Only numbers 1 through 9 are used.
    Each number is used at most once.

Return a list of all possible valid combinations. The list must not contain the same combination twice, and the combinations may be returned in any order.
*/

// Approach:
// We can use recursion and backtracking to solve this
// To make n with k digits, we need to fix one digit i then make n-i with k-1 digits
// 1) For the base case, if k is 1, return a 2d vector containing just the single requested digit if it is in the range 1-9
//    Otherwise return an empty 2d Vec
// 2) Make an empty 2d Vec to store the combinations
// 3) Loop i from k to n or k to 9 if n is > 9, where i will be the fixed digit
//    Then call combination_sum3 recursively with k-1 and n-i
//    Then we check of the return combinations, which ones are valid i.e. in which ones all elements are < i
//    And for all such combinations we find, we push them to the combinations vector
// 4) At the end we can return the combinations made

impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        if k == 1 {
            return if (1..=9).contains(&n) {vec!(vec!(n))} else {vec!()};
        }
        let mut combinations = Vec::new();
        for i in (k..=n.min(9)).rev() {
            let mut rest = Self::combination_sum3(k-1,n-i);
            for combination in rest.iter().filter(|&y| y.iter().all(|&z| z < i)) {
                let mut combi = combination.clone();
                combi.push(i);
                combinations.push(combi);
            }
        }
        combinations
    }
}
