pub struct Solution;

impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        let mut open = std::collections::VecDeque::new();
        let mut star = std::collections::VecDeque::new();

        // Phase 1: Use stars as opening brackets to match all closing brackets
        for (idx, c) in s.bytes().enumerate() {
            match c {
                b'(' => {
                    open.push_back(idx);
                }

                b')' => {
                    if let None = open.pop_back() {
                        if let None = star.pop_back() {
                            return false;
                        }
                    }
                }

                _ => {
                    star.push_back(idx);
                }
            }
        }

        // Phase 2: Use stars as closing brackets to match all remaining opening brackets
        let mut open_count = 0;
        while let Some(open_idx) = open.front()
            && let Some(star_idx) = star.front()
        {
            if open_idx < star_idx {
                open.pop_front();
                open_count += 1;
            }
            else {
                star.pop_front();
                if open_count > 0 {
                    open_count -= 1;
                }
            }
        }

        // Not open.is_empty() => there are unmatched '(' and no more stars after => invalid
        // open_count < star.len() => there are more star after all '(' is counted => valid
        open.is_empty() && open_count < star.len()
    }

    /*
    More efficient solution from a Leetcode user:
    - We maintain two variables:
        `low` => minimum possible number of unmatched '('
        `high` => maximum possible number of unmatched '('

    The * character can represent: '(' or ')' or empty string

    So for every character:
    - If it is '(', both low and high increase.
    - If it is ')', both decrease.
    - If it is '*', low decreases (treat * as ')' => close any possible remaining '(')
    and high increases (treat * as '(' => for any future ')').

    If `high` becomes negative, there are more closing brackets than we can possibly match
    => Because the max number of unmatched '(' is not high enough to match all ')'
    => Immediately return false.

    `low` can become negative because * can also represent an empty string.
    Therefore, we reset it to 0.

    At the end, if `low` == 0, there is a valid way to interpret the * characters.
    Otherwise, `low` > 0 => min number of unmatched '(' is too high, can't close them all.
    */
    pub fn check_valid_string_2(s: String) -> bool {
        let mut low: usize = 0;
        let mut high: isize = 0;

        for c in s.bytes() {
            match c {
                b'(' => {
                    low += 1;
                    high += 1;
                }

                b')' => {
                    low = low.saturating_sub(1);
                    high -= 1;
                }

                _ => {
                    low = low.saturating_sub(1);
                    high += 1;
                }
            }

            if high < 0 {
                return false;
            }
        }

        low == 0
    }
}
