pub struct Solution;

// https://leetcode.com/problems/maximize-sum-of-squares-of-digits
impl Solution {
    pub fn max_sum_of_squares(mut num: i32, mut sum: i32) -> String {
        // num is the number of digits
        // sum is the sum of the digits

        // The number of digits is not enough to make up the sum
        if num * 9 < sum {
            return "".to_owned();
        }

        // Find number with greatest sum of squared digits
        // -> Greedily assign max value (9) to the most significant digits first
        let mut result = String::with_capacity(num as usize);
        
        while sum > 0 {
            let next_digit = std::cmp::min(9, sum);
            sum -= next_digit;
            result.push((next_digit as u8 + b'0') as char);
            num -= 1;
        }

        while num > 0 {
            result.push('0');
            num -= 1;
        }

        result
    }

    // Much more efficient answer from Leetcode
    pub fn max_sum_of_squares_2(num: i32, sum: i32) -> String {
        // Not enough digits to make up the sum
        if sum > 9 * num {
            return String::new();
        }

        // Greedily take 9 digits for most significant digits
        // -> Because it should be all 9's anyway
        let q = sum / 9; // Number of 9 takens
        let r = sum % 9; // The digit after the 9's

        let mut ans = "9".repeat(q as usize);
        if r > 0 {
            ans.push((b'0' + r as u8) as char);
        }
        ans.push_str(&"0".repeat(num as usize - ans.len()));
        ans
    }
}