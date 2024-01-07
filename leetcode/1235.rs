/*
Question:
We have n jobs, where every job is scheduled to be done from startTime[i] to endTime[i], obtaining a profit of profit[i].

You're given the startTime, endTime and profit arrays,
return the maximum profit you can take such that there are no two jobs in the subset with overlapping time range.

If you choose a job that ends at time X you will be able to start another job that starts at time X.
*/

// Appraoch:
// 1) Make a struct for jobs
// 2) Make a binary search function
//    It will find and return the smallest index in the set that is <= a given index x
// 3) Create a vector of jobs and add all the jobs to it
// 4) Make a BTReeSet to store the jobs. The first value will be the time and the second will be
//    the maximum profit at that time
// 5) Sort the jobs by end time
// 6) Iterate through the jobs
//     For each job, calculate the profit if the job is taken
//     If the profit is greater than the last profit, add it to the set and update the last profit
// 7) Return the last profit

use std::collections::BTreeSet;

impl Solution {
    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        #[derive(Debug, Clone)]
        struct Job {
            start: i32,
            end: i32,
            profit: i32,
        }
        impl Job {
            fn new(start: i32, end: i32, profit: i32) -> Job {
                Job { start, end, profit }
            }
        }
        fn b_search(set: &BTreeSet<(i32, i32)>, x: i32) -> i32 {
            if let Some(&(i, n)) = set.range(..=(&(x, std::i32::MAX))).rev().next() {
                n
            } else {
                0
            }
        }
        let mut jobs = vec![];
        for i in 0..start_time.len() {
            jobs.push(Job::new(start_time[i], end_time[i], profit[i]));
        }
        jobs.sort_unstable_by_key(|j| j.end);
        let mut dp = BTreeSet::new();
        let mut last = 0;
        let mut j = 0;
        for job in &jobs {
            if j >= jobs.len() {
                continue;
            }
            let profit = b_search(&dp, job.start) + job.profit;
            if profit > last {
                dp.insert((job.end, profit));
                last = profit;
            }
        }
        dp.pop_last().unwrap().1
    }
}
