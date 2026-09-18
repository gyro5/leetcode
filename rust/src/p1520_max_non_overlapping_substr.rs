#[derive(Debug)]
pub struct Solution;

#[allow(unused)]
impl Solution {
    pub fn max_num_of_substrings(st: String) -> Vec<String> {
        let chars: Vec<char> = st.chars().collect();

        // Build a map of first and last occurances of each character
        let mut char_map = vec![vec![]; 26];

        for (i, &c) in chars.iter().enumerate() {
            let c_idx = c as usize - 'a' as usize;
            if char_map[c_idx].len() != 0 {
                continue;
            }

            // Find the last occurence of c
            let mut j = chars.len() - 1;
            while chars[j] != c {
                j -= 1;
            }
            char_map[c_idx].push(i);
            char_map[c_idx].push(j);
        }

        // Collect the interval into a set
        let mut intervals = std::collections::HashSet::new();
        for idx in 0..26 {
            if char_map[idx].len() == 0 {
                continue;
            }

            // Find the full valid substring by unioning all chars' intervals
            let mut s: usize = char_map[idx][0];
            let mut f: usize = char_map[idx][1];

            // A queue to keep track of all chars to-be-expanded
            let mut queue = std::collections::VecDeque::new();
            for j in s+1..f {
                queue.push_back(chars[j]);
            }

            while !queue.is_empty() {
                let bound_c = &char_map[queue.pop_back().unwrap() as usize - 'a' as usize];

                if bound_c.len() != 0 {
                    if bound_c[0] < s {
                        chars[bound_c[0]..s]
                            .iter()
                            .for_each(|&c1| queue.push_back(c1));
                        s = bound_c[0];
                    }
                    if bound_c[1] > f {
                        chars[f + 1..=bound_c[1]]
                            .iter()
                            .for_each(|&c1| queue.push_back(c1));
                        f = bound_c[1];
                    }
                }
            }

            intervals.insert((s, f, f - s + 1));
        }

        // NOTE: the intervals (aka valid substrings) cannot partially overlap due
        // to how the problem works --> They can only be completely inside of the other.

        // Sort the intervals by length
        let mut sorted_intervals: Vec<(usize, usize, usize)> = intervals.into_iter().collect();
        sorted_intervals.sort_by_key(|&(_, _, l)| l);

        let mut choosen: Vec<(usize, usize)> = vec![];
        // println!("{:?}", sorted_intervals);
        for (s, f, _) in sorted_intervals {
            let mut overlap = false;
            for &(s1, f1) in choosen.iter() {
                // Check for overlap -> See NOTE above
                if (s < s1 && s1 < f) || (s1 < s && s < f1) {
                    overlap = true;
                    break;
                }
            }
            if overlap {
                continue;
            }

            choosen.push((s, f));
        }

        println!("{:?}", choosen);

        choosen
            .into_iter()
            .map(|(s, f)| (&st[s..=f]).to_string())
            .collect()
    }
}
