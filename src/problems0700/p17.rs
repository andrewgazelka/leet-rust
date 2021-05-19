use crate::Solution;

impl Solution {
    pub fn is_one_bit_character(bits: Vec<i32>) -> bool {
        let len = bits.len();

        if len <= 1 {
            return true;
        }

        for i in (0..len - 1).rev() {
            if bits[i] == 0 {
                return (len - i) % 2 == 0;
            }
        }
        len % 2 == 1
    }
}
