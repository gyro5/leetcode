pub struct Solution;

/*
https://leetcode.com/problems/maximum-nesting-depth-of-two-valid-parentheses-strings/

NOTE: this problem is about how to split a valid parenthesis string into 2
subsequences with minimum max nested depth.

=> The idea is to use a common stack to keep track of which closing ) belongs to which
opening (. Then, we want to distribute the nesting depth of the original string to the
2 subsequences => by assigning even depth to the first subsequence and odd depth
to the second subsequence. (See alternative solution & explantion  below).
*/
impl Solution {
    pub fn max_depth_after_split(seq: String) -> Vec<i32> {
        let mut common_stack = std::collections::VecDeque::new();

        seq.bytes()
            .map(|c| {
                let i;
                if c == b'(' {
                    // Select the one with lower depth
                    i = common_stack.len() % 2;
                    common_stack.push_back(i);
                }
                else {
                    // Match closing bracket
                    i = common_stack.pop_back().unwrap();
                }
                i as i32
            })
            .collect()
    }
}

/*
My solution above is optimal time-complexity-wise (O(n)), but here
is a better solution in terms of space complexity (2.3 MB vs 2 MB):

pub fn max_depth_after_split(seq: String) -> Vec<i32> {
    seq.chars()
        .enumerate()
        .map(|(i, c)| {
            (i as i32 & 1) ^ if c == '(' { 1 } else { 0 }
        })
        .collect()
}

Explanation:
- `(i as i32 & 1)`: is 0 if even index, 1 if odd index
- `if c == '(' { 1 } else { 0 }`: 1 if opening, 0 is closing

=> The original parenthesis string is valid, so the indices of matching
opening and closing parentheses must contain both even and odd.
   For example: (__) => 0__3

=> So this is a way to divide the matching brackets evenly, by giving all
even-index opening ( to one subsequence, and giving all odd-index opening
( to the other.

Formal proof from Leetcode:

For an opening parenthesis '(':
    Let its 0-based index be x and its nesting depth be y. If there are l opening
    parentheses and r closing parentheses preceding it in the string:

        According to the definition of nesting depth: y = l − r + 1
        The 0-based index x is the total count of characters preceding it: x = l + r

    Adding the two equations yields x + y = 2l + 1, which is always odd.
    Therefore, the index x and the nesting depth y must have opposite parity.

For a closing parenthesis ')':
    Let its 0-based index be x and its nesting depth be y. If there are l opening
    parentheses and r closing parentheses preceding it:

        The nesting depth is: y = l − r
        The 0-based index is: x = l + r

    Subtracting the two equations yields x − y = 2r, which is always even. Therefore,
    the index x and the nesting depth y must have the same parity.

=> This allows us to determine the nesting depth of each parenthesis in the original
string, and we distribute this depth into the 2 subsequences (using odd/even aka
parity).
*/
