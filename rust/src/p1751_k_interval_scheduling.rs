#[derive(Debug)]
pub struct Solution;

use std::cmp;

#[allow(unused)]
impl Solution {
    pub fn max_value(mut events: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = events.len();
        let k = k as usize;

        // IMPORTANT!: need to sort by start time
        events.sort_by_key(|e| e[0]);

        // succ[i]: successor interval of interval i
        // aka closest starting time after finishing time of i
        let mut succ = vec![];
        for i in 0..n {
            // Note: depending on the condition of whether we can attend
            // event starting at time t right after finishing event ending
            // at time t. This case, we can't, hence `<=`.
            succ.push(events.partition_point(|e| e[0] <= events[i][1]));
        }

        // dp[i][j]: max schedule for suffix [i:] with max k events
        let mut dp = vec![vec![0; k + 1]; n + 1];
        for i in (0..=n).rev() {
            // Base case 1: No event left
            if i >= n {
                continue;
            }

            for j in (1..=k) {
                // Base case 2: No more event after this event
                if succ[i] == n {
                    dp[i][j] = events[i][2];
                }

                // Relate: Choose not attending vs attending this event
                // println!("{i}, {j}");
                dp[i][j] = cmp::max(
                    dp[i + 1][j],                      // Not attend -> Number of events stays the same
                    events[i][2] + dp[succ[i]][j - 1], // Attend -> j-1 event from successor
                );
            }
        }

        // println!("{:?}", succ);
        // println!("{:?}", dp);

        dp[0][k]
    }
}
