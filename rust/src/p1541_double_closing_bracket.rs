pub struct Solution;

impl Solution {
    pub fn min_insertions(s: String) -> i32 {
        let ss = s.replace("))", "]");

        let mut unmatched_open = 0; // = 2 * number of ) to add
        let mut unmatched_close = 0; // = number of ( to add
        let mut extra_close = 0; // to complete ) -> ))
        
        for c in ss.bytes() {
            if c == b'(' {
                unmatched_open += 1;
            }
            else if c == b']' {
                if unmatched_open > 0 {
                    unmatched_open -= 1;
                }
                else {
                    unmatched_close += 1;
                }
            }
            else {
                extra_close += 1; // To complete the current '))'
                if unmatched_open > 0 {
                    unmatched_open -= 1;
                }
                else {
                    unmatched_close += 1;
                }
            }
        }

        unmatched_open * 2 + unmatched_close + extra_close
    }

    /*
    A more efficient solution inspired from a Leetcode user
    Notably:
    - Use peekable iterator
    - Use `next_if` to only advance if match
    */
    pub fn min_insertions_2(s: String) -> i32 {
        let mut s_chars_iter = s.bytes().peekable();

        let mut unmatched_open = 0;
        let mut bracket_to_add = 0;

        while let Some(c) = s_chars_iter.next() {
            if c == b'(' {
                // Need 2 closes to match this open
                unmatched_open += 1;
            }
            else {
                if s_chars_iter.next_if_eq(&b')').is_none() {
                    bracket_to_add += 1; // To complete the current close
                }
                // Now that we have full )), check if it can match with an open
                if unmatched_open > 0 {
                    unmatched_open -= 1;
                }
                else {
                    // Need to add an opening bracket
                    bracket_to_add += 1;
                }
            }
        }

        unmatched_open * 2 + bracket_to_add
    }
}