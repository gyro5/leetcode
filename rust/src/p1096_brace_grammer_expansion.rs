pub struct Solution;

#[derive(Debug)]
enum Component {
    Single(String),
    List(Vec<Component>),
    Concat(Box<Component>, Box<Component>),
    EOF,
}

use std::collections::HashSet;

use Component::*;

// https://leetcode.com/problems/brace-expansion-ii/
// This problem is a just a whole parser and interpreter :)
impl Solution {
    /*
    Meta-grammar rules for the brace grammar:
    expr = concat
    concat -> component concat | EOF
    list -> "{" concat ("," concat)* "}"
    component -> STRING | list
    */
    pub fn brace_expansion_ii(expression: String) -> Vec<String> {
        let chars: Vec<u8> = expression.bytes().collect();
        let mut idx = 0;
        let mut res = Solution::evaluate(Solution::parse_concat(&expression, &chars, &mut idx));
        res.sort();
        res
    }

    fn try_match(chars: &[u8], idx: &mut usize, target: u8) -> bool {
        if chars[*idx] == target {
            *idx += 1;
            true
        }
        else {
            false
        }
    }

    fn parse_concat(orig: &str, chars: &[u8], idx: &mut usize) -> Component {
        if *idx >= chars.len() || chars[*idx] == ',' as u8 || chars[*idx] == '}' as u8 {
            EOF
        }
        else {
            // Right recursion so it's fine
            Concat(
                // Left argument
                Box::new(if Solution::try_match(chars, idx, '{' as u8) {
                    Solution::parse_list(orig, chars, idx)
                }
                else {
                    Solution::parse_single(orig, chars, idx)
                }),
                // Right argument
                Box::new(Solution::parse_concat(orig, chars, idx)),
            )
        }
    }

    fn parse_list(orig: &str, chars: &[u8], idx: &mut usize) -> Component {
        // Assume the opening brace is already consumed
        // Parse first component
        let mut res = vec![Solution::parse_concat(orig, chars, idx)];

        // Parse 0 or more other components
        while Solution::try_match(chars, idx, ',' as u8) {
            res.push(Solution::parse_concat(orig, chars, idx));
        }

        // Check the closing brace
        if !Solution::try_match(chars, idx, '}' as u8) {
            panic!("Incorrect syntax: missing closing brace");
        }

        List(res)
    }

    fn parse_single(orig: &str, chars: &[u8], idx: &mut usize) -> Component {
        let start = *idx;
        if start >= chars.len() {
            panic!("Should not happen")
        }
        else {
            while *idx < chars.len() && chars[*idx].is_ascii_alphabetic() {
                *idx += 1;
            }
            Single((&orig[start..*idx]).to_string())
        }
    }

    fn evaluate(expr: Component) -> Vec<String> {
        match expr {
            Single(s) => vec![s],

            List(components) => {
                let mut set: HashSet<String> =
                    HashSet::from_iter(components.into_iter().flat_map(Solution::evaluate));
                set.into_iter().collect()
            }

            Concat(a, b) => {
                let left = Solution::evaluate(*a);
                if let EOF = *b {
                    left
                }
                else {
                    let right = Solution::evaluate(*b);
                    left.iter()
                        .flat_map(|s_a| {
                            right.iter().map(|s_b| {
                                let mut s = String::with_capacity(s_a.len() + s_b.len());
                                s.push_str(s_a);
                                s.push_str(s_b);
                                s
                            })
                        })
                        .collect()
                }
            }

            EOF => vec![],
        }
    }
}
