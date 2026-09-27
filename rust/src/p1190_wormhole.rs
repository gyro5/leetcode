pub struct Solution;

/*
My idea is O(n^2), but Leetcode has a better O(n) solution:

Pass 1: Traverse the string, use a stack to track and store pairs of indices
for matching '(' and ')'.

Pass 2: Traverse the string again, adding characters one-by-one to a new string.
When reach a '(' or ')', teleport to the other end and change direction.
(left-right <=> right-left).
*/

// https://leetcode.com/problems/reverse-substrings-between-each-pair-of-parentheses
impl Solution {
    pub fn reverse_parentheses(s: String) -> String {
        let mut stack = std::collections::VecDeque::new();
        let mut pairs = std::collections::HashMap::new();

        let chars = s.as_bytes();

        // Pass 1: Find matching brackets
        for (idx, c) in chars.iter().enumerate() {
            match *c {
                b'(' => {
                    stack.push_back(idx);
                }

                b')' => {
                    let opening = stack.pop_back().unwrap();
                    pairs.insert(opening, idx);
                    pairs.insert(idx, opening);
                }

                _ => {}
            }
        }

        // Pass 2: Traverse the string again
        // res should have all characters in s minus the parentheses
        let mut res = String::with_capacity(s.len() - pairs.len());
        let mut idx = 0;
        let mut direction: i32 = 1;

        while idx < chars.len() {
            let c = chars[idx];
            match c {
                /*
                Explanation for why this works:
                ...->(.........)......  go into '('
                .....(.......<-)......  teleport to ')'
                .....(<-.......)......  move along, then go into '('
                .....(.........)->....  teleport to ')' with the correct direction

                => Think of each parenthesis as a magic 2-side portal, with each
                side connecting to one side of the matching parenthesis.
                */
                b'(' | b')' => {
                    idx = *pairs.get(&idx).unwrap();
                    direction = -direction;
                }

                _ => res.push(c as char)
            }
            idx = (idx as i32 + direction) as usize;
        }

        res
    }
}
