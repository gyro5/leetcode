pub struct Solution;

impl Solution {
    pub fn smallest_index(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .map(|mut num| {
                let mut digit_sum = 0;
                while num > 0 {
                    digit_sum += num % 10;
                    num /= 10;
                }
                digit_sum
            })
            .enumerate()
            .skip_while(|(idx, sum_digit)| *idx as i32 != *sum_digit)
            .next()
            .map_or(-1, |(idx, _)| idx as i32)
    }
}
