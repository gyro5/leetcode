pub struct Solution;

// https://leetcode.com/problems/minimum-operations-to-reduce-x-to-zero
#[allow(unused)]
impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        // This problem is equivalent to finding the max-length subarray of nums that
        // has sum = total - x.

        // Approach: **Sliding window**
        // - Keep a running sum while advancing the right pointer
        // - Shrink window from the left when the sum exceeds target
        // - Record the current maximum length whenever the sum exactly matches target
        //
        // => NOTE: This approach is possible because every element is positive.

        let target = nums.iter().sum::<i32>() - x;

        let mut l = 0;
        let mut curr_sum = 0;
        let mut curr_max: i32 = -1;

        // The order of operations in the loop is very important for the correctness:
        // -> Invariant: at the end of each iteration, curr_sum = sum[l..=r] for
        //    the current r and for l such that the sum is <= target.
        for (r, item) in nums.iter().enumerate() {
            curr_sum += item;
            while curr_sum > target && l < nums.len() {
                curr_sum -= nums[l];
                l += 1;
            }

            if curr_sum == target {
                // Check the sum from the previous iteration
                curr_max = curr_max.max((r - l + 1) as i32);
            }
        }

        if curr_max > -1 {
            nums.len() as i32 - curr_max
        }
        else {
            -1
        }
    }
}
