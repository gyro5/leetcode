pub struct Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut a_chars = a.bytes().rev();
        let mut b_chars = b.bytes().rev();
        let mut carry = 0;

        let mut res: Vec<u8> = (&mut a_chars).zip(&mut b_chars).map(|(a_c, b_c)| {
            let sum = (a_c - b'0') + (b_c - b'0') + carry;
            carry = (sum & 0b10) >> 1;
            (sum & 0b1) + b'0'
        }).collect();

        // Remaining digits
        for a_c in a_chars {
            let sum = (a_c - b'0') + carry;
            carry = (sum & 0b10) >> 1;
            res.push((sum & 0b1) + b'0');
        }
        for b_c in b_chars {
            let sum = (b_c - b'0') + carry;
            carry = (sum & 0b10) >> 1;
            res.push((sum & 0b1) + b'0');
        }

        // Final carry
        if carry == 1 {
            res.push(b'1');
        }
        res.reverse();

        String::from_utf8(res).unwrap()
    }
}
