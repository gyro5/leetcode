pub struct Solution;

use std::collections::HashMap;

/*
This problem is easy but it highlights some good practice,
shown in "NOTE" below.
*/
impl Solution {
    pub fn evaluate(s: String, knowledge: Vec<Vec<String>>) -> String {
        // Store the string mapping into a HashMap
        //
        // NOTE: knowledge (and its strings) is not moved to map,
        // the map is built in one expression by mapping to tuple.
        // Note how the map contains only slice, not actual owned values.
        let map = knowledge
            .iter()
            .map(|pair| (pair[0].as_bytes(), pair[1].as_str()))
            .collect::<HashMap<_, _>>();

        // Parse the input string
        //
        // NOTE: Instead of collecting into Vec<String>, we can just
        // collect to a string allocated with enough capacity at the start
        let chars = s.as_bytes();
        let mut idx = 0;
        let mut res = String::with_capacity(s.len());

        while idx < chars.len() {
            if chars[idx] == b'(' {
                let start = idx + 1;
                idx += 1;

                // Parse the to-be-replaced key
                while idx < chars.len() && chars[idx].is_ascii_alphabetic() {
                    idx += 1;
                }

                // At this point, idx should point to the closing )
                res.push_str(map.get(&chars[start..idx]).map_or("?", |v| v));
                idx += 1;
            }
            else {
                let start = idx;

                // Parse the normal string
                while idx < chars.len() && chars[idx].is_ascii_alphabetic() {
                    idx += 1;
                }

                res.push_str(&s[start..idx]);
            }
        }

        res
    }
}
