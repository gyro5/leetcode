pub struct Solution;

impl Solution {
    pub fn has_valid_path(grid: Vec<Vec<char>>) -> bool {
        let m = grid.len();
        let n = grid[0].len();

        // Some low-hanging-fruit base cases
        // 1. All path has length n + m - 1, must be even to be valid
        if (n + m - 1) % 2 != 0 {
            return false;
        }

        // 2. Must start with '(' and end with ')'
        if (grid[0][0] == ')') || (grid[m - 1][n - 1] == '(') {
            return false;
        }

        let mut dp: Vec<Vec<Vec<Option<bool>>>> = vec![vec![vec![None; n + m]; n]; m];

        Solution::dfs(&grid, &mut dp, 0, 0, 0, m, n)
    }

    pub fn dfs(
        grid: &Vec<Vec<char>>,
        dp: &mut Vec<Vec<Vec<Option<bool>>>>,
        mut stack: i32,
        i: usize,
        j: usize,
        m: usize,
        n: usize,
    ) -> bool {
        // Base case for the DP, might not be in the vec dp because
        // stack can be negative (so not a valid index).
        // But return rightaway so it's no problem.

        // 1. Outside the grid
        if i >= m || j >= n || stack < 0 {
            return false;
        }

        // 2. At exactly the bottom-right box -> Check stack after visit current char
        let next = stack + (if grid[i][j] == '(' { 1 } else { -1 });
        if i == m - 1 && j == n - 1 {
            return next == 0;
        }

        // NOTE: It's very important to pass "next" instead of "stack"
        if dp[i][j][stack as usize].is_none() {
            // Dfs to bottom or right node, short-circuit here to save time
            dp[i][j][stack as usize] = Some(
                Solution::dfs(grid, dp, next, i + 1, j, m, n)
                    || Solution::dfs(grid, dp, next, i, j + 1, m, n),
            );
        }

        dp[i][j][stack as usize].unwrap()
    }
}
