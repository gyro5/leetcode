// https://leetcode.com/problems/score-of-parentheses
pub struct Solution;

use std::ops::AddAssign;

impl Solution {
    pub fn score_of_parentheses(s: String) -> i32 {
        // s is always a balanced parenthesis string
        let s_bytes = s.as_bytes();

        let mut stack = std::collections::VecDeque::new();
        stack.push_back(0);

        for (idx, &c) in s_bytes.iter().enumerate() {
            if c == b'(' {
                stack.push_back(0);
            }
            else {
                let mut curr = 0;
                if s_bytes[idx - 1] == b'(' {
                    // () -> 1
                    curr = 1;
                }
                else {
                    while *stack.back().unwrap() != 0 {
                        curr += stack.pop_back().unwrap();
                    }
                    curr *= 2;
                }

                // At this point, curr should contain the value of the
                // sub-expression enclosed by the current ')', e.g. "(...)"
                stack.back_mut().unwrap().add_assign(curr);
            }
        }

        *stack.back().unwrap()
    }

    // A more efficient solution from Leetcode:
    // Instead of trying to calculate during traversal,
    // this solution see the whole expression in expanded form
    // 
    // => Each "()" will contribute 1 * 2 ^ depth to the final sum.
    // Ex: (()(()())) => (1 + (1 + 1) * 2) * 2
    //                => 1*2 + 1*2^2 + 1*2^2
    pub fn score_of_parentheses_2(s: String) -> i32 {
        let bytes = s.as_bytes();
        let mut depth = 0;
        let mut score = 0;

        for i in 0..bytes.len() {
            if bytes[i] == b'(' {
                depth += 1;
            }
            else {
                depth -= 1;

                // The expanded value of each "()" is 1 * 2 ^ depth
                if i > 0 && bytes[i - 1] == b'(' {
                    score += 1 << depth;
                }
            }
        }
        score
    }
}
