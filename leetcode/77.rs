/*
Question:
Given two integers n and k, return all possible combinations of k numbers chosen from the range [1, n].
You may return the answer in any order.
*/

// Approach:
// We make combinations using recursion with the backtrack function, it works as follows
// 1) The base case is when the length of current combination(so_far) equals or exceeds the required length
// 2) Otherwise we loop from next(next is the next element to be added to combination) till n and push the number into so far
//    Then we recursively call backtrack with i+1 as next and all the other parameters same
// 3) After generating a combination this way, we remove the last element and then proceed to the next iteration of the loop

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        fn backtrack(next: usize, so_far: &mut Vec<i32>, combinations: &mut Vec<Vec<i32>>, n: usize, k: usize) {
            if so_far.len() >= k {
                combinations.push(so_far.clone());
                return;
            }
            for i in next..=n {
                so_far.push(i as i32);
                backtrack(i+1,so_far,combinations,n,k);
                so_far.pop();
            }
        }

        let mut so_far : Vec<i32> = Vec::new();
        let mut combinations : Vec<Vec<i32>> = Vec::new();
        backtrack(1, &mut so_far, &mut combinations, n as usize, k as usize);
        combinations
    }
}
