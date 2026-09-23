#[derive(Debug)]
pub struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let chars: Vec<u8> = s.as_bytes().into();
        let mut char_count: std::collections::HashMap<u8, usize> = std::collections::HashMap::new();

        // Use left and right pointers to maintain the sliding window
        let mut left = 0;
        let mut res = 0;

        // Loop: Keep advancing the right pointer and update the state
        // of the window. If the state becomes invalid: move the left
        // pointer until the state is valid again.
        for (right, ch) in chars.iter().enumerate() {
            // Increment count for character at "right"
            char_count.insert(*ch, char_count.get(ch).copied().unwrap_or(0) + 1);

            while char_count[ch] > 1 {
                *(char_count.get_mut(&chars[left]).unwrap()) -= 1;
                left += 1;
            }

            res = res.max(right - left + 1);
        }

        res as i32
    }
}
