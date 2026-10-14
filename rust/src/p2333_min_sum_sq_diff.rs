use std::ops::Deref;

pub struct Solution;

// https://leetcode.com/problems/minimum-sum-of-squared-difference
/*
The idea for this problem is:
- We want to minimize the squared differences, so k1 and k2 are the same.
- To minimize a^2 + b^2, just try to make a == b -> So we can just greedily
reduce the highest differences.
- Only the differences matter, so we can just calculate that from the start.
*/
impl Solution {
    pub fn min_sum_square_diff(nums1: Vec<i32>, nums2: Vec<i32>, k1: i32, k2: i32) -> i64 {
        let mut diffs: Vec<usize> = nums1
            .iter()
            .zip(nums2.iter())
            .map(|(a, b)| a.abs_diff(*b) as usize)
            .collect();

        let max_diff = *(diffs.iter().max().unwrap()) as usize;
        if max_diff == 0 {
            // Short cut for this edge case
            return 0;
        }

        // Because the diff cannot exceed 100000, we can use buckets
        let mut counts = vec![0; max_diff + 1];
        for d in diffs {
            counts[d as usize] += 1;
        }

        // Greedily reduce the diff
        let mut total_moves = (k1 + k2) as usize;
        for idx in (1..=max_diff).rev() {
            if total_moves == 0 {
                break;
            }
            if counts[idx] == 0 {
                continue;
            }
            else {
                // Reduce all current diff values by 1
                let update = total_moves.min(counts[idx]);
                total_moves -= update;
                counts[idx] -= update;
                counts[idx - 1] += update;
            }
        }

        counts
            .iter()
            .enumerate()
            .map(|(val, &count)| val * val * count)
            .sum::<usize>() as i64
    }
}
