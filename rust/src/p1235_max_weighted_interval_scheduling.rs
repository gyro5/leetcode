#[derive(Debug)]
pub struct Solution;

#[allow(unused)]
use std::cmp;

#[allow(unused)]
impl Solution {
    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        let n = start_time.len();

        // Build a tuple list and sort by start time
        let mut data = vec![];
        for i in 0..n {
            data.push((start_time[i], end_time[i], profit[i]));
        }
        data.sort_by_key(|&(s, _, _)| s); // IMPORTANT!

        // succ[i]: successor interval of interval i
        // aka closest starting time after finishing time of i
        let mut succ = vec![];
        for i in 0..n {
            succ.push(data.partition_point(|&(s, _, _)| s < data[i].1));
        }

        // dp[i]: max schedule for suffix [i:]
        let mut dp = vec![0; n + 1];
        for i in (0..=n).rev() {
            // Base case 1: No more job
            if i >= n {
                continue;
            }

            // Base case 2: No job left after this
            if succ[i] == n {
                dp[i] = data[i].2;
            }

            // Relate: Choose don't do job i vs do job i
            dp[i] = cmp::max(
                dp[i + 1],               // Don't -> Check the suffix [i+1:]
                data[i].2 + dp[succ[i]], // Do -> Get this job and the suffix [succ[i]:]
            );
        }

        dp[0]
    }
}
