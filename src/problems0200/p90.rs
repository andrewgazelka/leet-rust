use crate::Solution;
use crate::utils::mask;
use std::ops::Not;

const MASK_8: u32 = mask(8);
const MASK_4: u32 = mask(4);
const MASK_2: u32 = mask(2);
const MASK_1: u32 = mask(1);

impl Solution {
    pub fn reverse_bits(mut n: u32) -> u32 {

        // first 16 bits and last 16 bits get swapped
        n = (n >> 16) | (n << 16);

        // swapping each 8 bits
        n = ((n & MASK_8.not()) >> 8) | ((n & MASK_8) << 8);

        // swapping each 4 bits
        n = ((n & MASK_4.not()) >> 4) | ((n & MASK_4) << 4);

        // swapping each 2 bits
        n = ((n & MASK_2.not()) >> 2) | ((n & MASK_2) << 2);

        // swapping each bit
        n = ((n & MASK_1.not()) >> 1) | ((n & MASK_1) << 1);

        n
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        let a = 0b111100110000;

        println!("{:b}", Solution::reverse_bits(a));
    }
}
