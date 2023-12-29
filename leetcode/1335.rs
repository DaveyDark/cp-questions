/*
Question:
You want to schedule a list of jobs in d days.
Jobs are dependent (i.e To work on the ith job,
you have to finish all the jobs j where 0 <= j < i).

You have to finish at least one task every day.
The difficulty of a job schedule is the sum of difficulties of each day of the d days.
The difficulty of a day is the maximum difficulty of a job done on that day.

You are given an integer array jobDifficulty and an integer d.
The difficulty of the ith job is jobDifficulty[i].

Return the minimum difficulty of a job schedule.
If you cannot find a schedule for the jobs return -1.
*/

// Approach:
// 1) If the number of jobs is less than the number of days, return -1
// 2) If the sum of all the job difficulties is 0, return 0
// 3) Make a memoization table of size d x job_difficulty.len()
// 4) Make a recursive function split() that takes the job_difficulty, d, and the current index
// 5) In the split() function, if d == 1, return the max of the job_difficulty[i..]
//    If the memo table has the value for d and i, return the value
//    Else, iterate from i to job_difficulty.len() - d and find the optimal split and return it
// 6) Call the split() function with the job_difficulty, d, and 0 as the current index

impl Solution {
    pub fn min_difficulty(job_difficulty: Vec<i32>, d: i32) -> i32 {
        if (job_difficulty.len() as i32) < d {
            return -1;
        } else if job_difficulty.iter().sum::<i32>() == 0 {
            return 0;
        }

        let mut memo: Vec<Vec<i32>> = vec![vec![-1; job_difficulty.len()]; d as usize];
        Self::split(&job_difficulty, d, 0, &mut memo)
    }

    fn split(jd: &Vec<i32>, d: i32, i: usize, memo: &mut Vec<Vec<i32>>) -> i32 {
        if d == 1 {
            memo[d as usize - 1][i] = *jd[i..].iter().max().unwrap();
            memo[d as usize - 1][i]
        } else if memo[d as usize - 1][i] != -1 {
            memo[d as usize - 1][i]
        } else {
            let mut min = i32::MAX;
            for j in i..=jd.len() - d as usize {
                let mut curr = *jd[i..=j].iter().max().unwrap();
                let other = Self::split(jd, d - 1, j + 1, memo);
                min = min.min(other + curr);
            }
            memo[d as usize - 1][i] = min;
            min
        }
    }
}
