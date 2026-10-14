pub struct Solution;

impl Solution {
    // O(nlogn) --> See below for O(n) solution
    pub fn max_alternating_sum(mut nums: Vec<i32>) -> i64 {
        nums.sort_by_key(|k| k.abs());
        nums[nums.len() / 2..nums.len()]
            .iter()
            .map(|&e| e as i64 * e as i64)
            .sum::<i64>()
            - nums[0..nums.len() / 2]
                .iter()
                .map(|&e| e as i64 * e as i64)
                .sum::<i64>()
    }

    // Alternative solution from leetcode, just O(n) instead of O(nlogn)
    // NOTE: the use of select_nth_unstable_by_key
    pub fn max_alternating_sum_2(mut nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let (lesser, &mut median, greater) =
            nums.select_nth_unstable_by_key(n / 2, |&num| num.abs());

        median as i64 * median as i64
            + greater
                .into_iter()
                .map(|&mut num| (num as i64) * (num as i64))
                .sum::<i64>()
            - lesser
                .into_iter()
                .map(|&mut num| (num as i64) * (num as i64))
                .sum::<i64>()
    }
}
