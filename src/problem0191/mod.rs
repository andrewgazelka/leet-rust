use crate::Solution;

const NUM_BITS: u32 = 32;

const fn create_mask_helper(mut input: u32, set: bool, width: u8, iter: u32, sub_iter: u8) -> u32 {
    input <<= 1;
    if set {
        input |= 0b1;
    }
    if iter == 0 {
        return input;
    }

    if sub_iter == 0 {
        create_mask_helper(input, !set, width, iter - 1, width)
    } else {
        create_mask_helper(input, set, width, iter - 1, sub_iter - 1)
    }
}

const fn create_mask(width: u8) -> u32 {
    let iter_0idx = NUM_BITS - 1;
    let width_0idx = width - 1;
    create_mask_helper(0, false, width_0idx, iter_0idx, width_0idx)
}

const MASK_1: u32 = create_mask(1);
const MASK_2: u32 = create_mask(2);
const MASK_3: u32 = create_mask(4);
const MASK_4: u32 = create_mask(8);
const MASK_5: u32 = create_mask(16);

impl Solution {
    pub fn hamming_weight_kernighan(mut n: u32) -> u32 {
        let mut c = 0_u32;
        while n > 0 {
            n &= n - 1;
            c += 1;
        }
        c
    }


    pub fn hamming_weight_logn(mut c: u32) -> u32 {
        // 1 0 1 1 0 1 1 0
        c = ((c >> 1) & MASK_1) + (c & MASK_1);

        // 1 0 1 1 0 1 1 0  &    (INPUT)
        // 0 1 0 1 0 1 0 1
        // 0 0 0 1 0 1 0 0  =    (A)

        // 0 1 0 1 1 0 1 1  &
        // 0 1 0 1 0 1 0 1
        // 0 1 0 1 0 0 0 1  =    (B)

        // 0 0 0 1 0 1 0 0       (A)
        // 0 1 0 1 0 0 0 1  +    (B)
        // 0 1 1 0 0 1 0 1  +    (B)

        c = ((c >> 2) & MASK_2) + (c & MASK_2);
        c = ((c >> 4) & MASK_3) + (c & MASK_3);
        c = ((c >> 8) & MASK_4) + (c & MASK_4);
        c = ((c >> 16) & MASK_5) + (c & MASK_5);
        c
    }
}


#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        let res = Solution::hamming_weight_logn(0b010011010);
        assert_eq!(res, 4);
    }
}
