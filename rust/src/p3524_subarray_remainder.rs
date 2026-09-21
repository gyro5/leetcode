#[derive(Debug)]
pub struct Solution;

#[allow(unused)]
use std::cmp;

// https://leetcode.com/problems/find-x-value-of-array-i/
#[allow(unused)]
impl Solution {
    pub fn result_array(nums: Vec<i32>, k: i32) -> Vec<i64> {
        let k = k as usize;

        // dp[r][i]: Number of subarrays starting at i that has
        // its product mod k = r. (r = 0..=k-1)
        let mut dp = vec![vec![0; nums.len() + 1]; k as usize];

        // Base case: No item left -> No subarray.

        // Relate: Subarrays starting at i -> nums[i] is in the product
        // --> Just multiply with subarrays starting at i+1 to get the mod
        for i in (0..nums.len()).rev() {
            // Consider the subarray containing only the current item first
            dp[nums[i] as usize % k][i] = 1;

            // For each total count of [i+1:], calculate the mod when times with nums[i]
            // (current item) and update the corresponding total count for [i:]
            for r in 0..k {
                dp[r * nums[i] as usize % k][i] += dp[r][i + 1];
            }
        }

        dp.iter().map(|arr| arr.iter().sum()).collect()
    }
}
