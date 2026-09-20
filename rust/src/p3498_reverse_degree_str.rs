#[derive(Debug)]
pub struct Solution;

#[allow(unused)]
impl Solution {
    pub fn reverse_degree(s: String) -> i32 {
        s.bytes()
            .enumerate()
            .map(|(idx, c)| (idx as i32 + 1) * (26 - (c as i32 - 'a' as i32)))
            .sum()
    }
}
