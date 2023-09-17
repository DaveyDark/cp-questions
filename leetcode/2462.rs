/*
Question:
You are given a 0-indexed integer array costs where costs[i] is the cost of hiring the ith worker.

You are also given two integers k and candidates. We want to hire exactly k workers according to the following rules:
    You will run k sessions and hire exactly one worker in each session.
    In each hiring session, choose the worker with the lowest cost from either the first candidates workers or the last candidates workers. Break the tie by the smallest index.
        For example, if costs = [3,2,7,7,1,2] and candidates = 2, then in the first hiring session, we will choose the 4th worker because they have the lowest cost [3,2,7,7,1,2].
        In the second hiring session, we will choose 1st worker because they have the same lowest cost as 4th worker but they have the smallest index [3,2,7,7,2]. Please note that the indexing may be changed in the process.
    If there are fewer than candidates workers remaining, choose the worker with the lowest cost among them. Break the tie by the smallest index.
    A worker can only be chosen once.

Return the total cost to hire exactly k workers.
*/

// Question:
// We push the negative of costs to the heaps since BinaryHeap is a max heap but we need a min heap
// 1) Make two heaps to store the first and last candidates and two ints to track the indices of
//    the front and last and also a counter for the total cost
// 2) Push the first candidates elements into the first heap
// 3) Push candidates elements from the last into the last heap until i <= j
// 4) Loop k times
//      If first is empty or last is non-empty and it's top is less than the top of first, pop
//      an element from last and add it to the total cost, then also push anohter element if i <= j
//      Otherwise pop an element from first and add it to toal_cost and push another element to it if i <= j
// 5) Return calculated total_cost

use std::collections::BinaryHeap;

impl Solution {
    pub fn total_cost(costs: Vec<i32>, k: i32, candidates: i32) -> i64 {
        let mut first = BinaryHeap::new();
        let mut last = BinaryHeap::new();
        let mut i = 0;
        let mut j = costs.len() as i32 - 1;
        let mut total_cost = 0;
        for _ in 0..candidates {
            first.push(-costs[i as usize]);
            i += 1;
        }
        for _ in 0..candidates {
            if i > j {break}
            last.push(-costs[j as usize]);
            j -= 1;
        }
        for _ in 0..k {
            if first.is_empty() || !last.is_empty() && -first.peek().unwrap() > -last.peek().unwrap() {
                total_cost -= last.pop().unwrap() as i64;
                if i <= j {
                    last.push(-costs[j as usize]);
                    j -= 1;
                }
            } else {
                total_cost -= first.pop().unwrap() as i64;
                if i <= j {
                    first.push(-costs[i as usize]);
                    i += 1;
                }
            }
        }
        total_cost
    }
}
