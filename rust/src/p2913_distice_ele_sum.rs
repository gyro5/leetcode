use std::collections::HashSet;

pub struct Solution;

// https://leetcode.com/problems/subarrays-distinct-element-sum-of-squares-i
impl Solution {
    pub fn sum_counts(nums: Vec<i32>) -> i32 {
        // 1-ele subarray always contribute 1
        let n = nums.len() as i32;
        let mut res = n;

        for i in 0..=n - 2 {
            for j in i + 2..=n {
                let set: HashSet<&i32> = HashSet::from_iter(&nums[i as usize..j as usize]);
                res += set.len().pow(2) as i32;
            }
        }

        res
    }

    // Better answer from a leetcode user
    pub fn sum_counts_2(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut ans = 0;

        for start in 0..n {
            // New count array for each starting index
            let mut used = vec![false; 101];
            let mut unique = 0;

            // Incremental count for all subarray starting at the same index
            // ie. nums[start..=start], nums[start..=start+1], ...
            for end in start..n {
                let i = nums[end] as usize;
                if !used[i] {
                    // If not seen, add to unique count and mark as seen
                    unique += 1;
                    used[i] = true;
                }
                ans += unique * unique;
            }
        }

        ans
    }
}
