pub struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut res = vec![];
        let mut temp = String::with_capacity(n as usize * 2);
        Solution::backtrack(&mut res, n, n, &mut temp);
        res
    }

    /*
    This problem is an example of backtracking, where we explore a
    "graph" of options, and backtrack to try other options when one option
    is done or invalid.
    */
    fn backtrack(res: &mut Vec<String>, open: i32, close: i32, temp: &mut String) {
        // Reach the end of this track -> Add result to res
        if open == 0 && close == 0 {
            res.push(temp.to_owned());
        }

        // Path 1: Add a new opening bracket
        if open > 0 {
            temp.push('(');
            Solution::backtrack(res, open - 1, close, temp);
            temp.pop();
        }

        // Path 2: Add a new closing bracket. Note how the condition essentially
        // closes all invalid paths (aka when the current string has no more
        // unmatched opening bracket to close).
        if close > open {
            temp.push(')');
            Solution::backtrack(res, open, close - 1, temp);
            temp.pop();
        }

        // Backtracking is done by the function returning to its caller,
        // then the caller will try the other path.
    }
}