pub struct Solution;

// https://leetcode.com/problems/longest-valid-parentheses
/*
NOTE:
This question is quite tricky and this is not my own solution.

The idea is to keep track of the indices of unmatched brackets, then the
substrings between those unmatched brackets will be valid substrings.
Then, we just need to keep track of the longest length of such substring.

The stack is used to both match matching brackets, and to keep track of any
unmatched bracket. For unmatched '(', their indices are pushed but not popped
by a matching ')'. For unmatched ')', their indices are pushed when there is
nothing in the stack to match them.
*/
impl Solution {
    // See a much clearer (and without the -1 trick) but slightly less
    // concise solution below.
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut max_len = 0;

        let s_bytes = s.as_bytes();
        let mut stack = std::collections::VecDeque::new();

        // This -1 acts as the index before the start of the string.
        stack.push_back(-1);

        // Invariant for the whole loop: Before and after each iteration, stack top
        // contains either the initial -1 or the index of the latest unmatched bracket.
        for (idx, &c) in s_bytes.iter().enumerate() {
            let idx = idx as i32;
            if c == b'(' {
                stack.push_back(idx);
            }
            else {
                // This will either pop the matching '(', or the initial -1,
                // or the index of the previous unmatched ')'.
                //
                // Note that is this pops a '(', the stack cannot be empty, because
                // before an unmatched '(', there must be either the initial -1 or
                // an unmatched ')'.
                stack.pop_back();

                // Invariant: if the current char is ')', the stack top is always
                // the index right before the start of the current valid substring
                // (aka the index of the previous unmatched '(' or ')' bracket.).
                //
                // An unmatched '(' can be matched and popped in a future iteration,
                // hence the index is peeked, not popped.
                if let Some(j) = stack.back() {
                    // Stack not empty -> s[j+1..=idx] is a valid substring
                    max_len = max_len.max(idx - j);
                }
                else {
                    // Stack is empty due to either the initial -1 or the
                    // previous unmatched ')' being popped.
                    stack.push_back(idx);
                }
            }
        }

        max_len as i32
    }

    // This version is like the above version but more explicit about
    // the logic. => See the else block in the loop.
    //
    // For this version, all unmatched brackets are kept in the stack
    // instead of replaced like the above solution.
    pub fn longest_valid_parentheses_2(s: String) -> i32 {
        let mut max_len = 0;
        let mut stack = std::collections::VecDeque::new();

        // Invariant: the stack only contains the unmatched brackets up to the
        // current index, with the stack top containing the latest unmatched bracket.
        // The indices of these unmatched brackets are separators between valid
        // substrings (including the empty substring).
        for (idx, c) in s.bytes().enumerate() {
            if c == b'(' {
                stack.push_back((c, idx));
            }
            else {
                if let Some((b'(', _)) = stack.back() {
                    // There is a matching '(' => Pop this
                    stack.pop_back();

                    // Then, the string from the index in the next stack top
                    // to the current index is a valid substring.
                    let start_valid = if let Some((_, last_unmatched)) = stack.back() {
                        last_unmatched + 1
                    }
                    else {
                        // No stack top => Valid from the very start
                        0
                    };

                    // This will be the length of the valid substring ending at
                    // the current index (which can only happen when the current
                    // index is a ')' with a matching '(').
                    max_len = max_len.max(idx - start_valid + 1);
                }
                else {
                    // This is an unmatched ')' -> Push this as a marker
                    // to the stack for a future iteration
                    stack.push_back((b')', idx));
                }
            }
        }

        max_len as i32
    }
}
